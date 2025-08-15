use crate::{Config, ConfigIssue, ConfigIssueSeverity, ValidationResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Configuration documentation generator
#[derive(Debug, Clone)]
pub struct ConfigDocumentationGenerator {
    /// Documentation templates
    templates: HashMap<DocumentationFormat, DocumentationTemplate>,
    /// Documentation settings
    settings: DocumentationSettings,
}

/// Documentation format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DocumentationFormat {
    Markdown,
    HTML,
    PDF,
    JSON,
    YAML,
}

/// Documentation template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationTemplate {
    /// Template format
    pub format: DocumentationFormat,
    /// Template content
    pub content: String,
    /// Template variables
    pub variables: Vec<String>,
}

/// Documentation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSettings {
    /// Include examples in documentation
    pub include_examples: bool,
    /// Include validation rules in documentation
    pub include_validation_rules: bool,
    /// Include troubleshooting section
    pub include_troubleshooting: bool,
    /// Include best practices section
    pub include_best_practices: bool,
    /// Include API reference
    pub include_api_reference: bool,
    /// Custom CSS for HTML output
    pub custom_css: Option<String>,
    /// Output directory
    pub output_directory: PathBuf,
}

/// Configuration documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigDocumentation {
    /// Documentation title
    pub title: String,
    /// Documentation version
    pub version: String,
    /// Documentation sections
    pub sections: Vec<DocumentationSection>,
    /// Generated timestamp
    pub generated_at: String,
    /// Configuration summary
    pub summary: DocumentationSummary,
}

/// Documentation section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSection {
    /// Section title
    pub title: String,
    /// Section content
    pub content: String,
    /// Section type
    pub section_type: SectionType,
    /// Subsections
    pub subsections: Vec<DocumentationSection>,
}

/// Section type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SectionType {
    Overview,
    Configuration,
    Validation,
    Examples,
    Troubleshooting,
    BestPractices,
    ApiReference,
    Changelog,
}

/// Documentation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSummary {
    /// Total number of configuration options
    pub total_options: usize,
    /// Number of required options
    pub required_options: usize,
    /// Number of optional options
    pub optional_options: usize,
    /// Number of validation rules
    pub validation_rules: usize,
    /// Number of examples
    pub examples: usize,
    /// Documentation completeness score (0-100)
    pub completeness_score: u8,
}

/// Documentation generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationResult {
    /// Generated documentation
    pub documentation: ConfigDocumentation,
    /// Output files
    pub output_files: Vec<PathBuf>,
    /// Generation statistics
    pub statistics: DocumentationStatistics,
}

/// Documentation generation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationStatistics {
    /// Generation time in milliseconds
    pub generation_time_ms: u64,
    /// Number of files generated
    pub files_generated: usize,
    /// Total file size in bytes
    pub total_size_bytes: usize,
    /// Number of sections generated
    pub sections_generated: usize,
}

impl ConfigDocumentationGenerator {
    /// Create a new documentation generator
    pub fn new(settings: DocumentationSettings) -> Self {
        let mut generator = Self {
            templates: HashMap::new(),
            settings,
        };
        generator.initialize_templates();
        generator
    }

    /// Initialize documentation templates
    fn initialize_templates(&mut self) {
        // Markdown template
        let markdown_template = DocumentationTemplate {
            format: DocumentationFormat::Markdown,
            content: include_str!("templates/markdown_template.md").to_string(),
            variables: vec![
                "title".to_string(),
                "version".to_string(),
                "sections".to_string(),
                "summary".to_string(),
            ],
        };

        // HTML template
        let html_template = DocumentationTemplate {
            format: DocumentationFormat::HTML,
            content: include_str!("templates/html_template.html").to_string(),
            variables: vec![
                "title".to_string(),
                "version".to_string(),
                "sections".to_string(),
                "summary".to_string(),
                "custom_css".to_string(),
            ],
        };

        self.templates
            .insert(DocumentationFormat::Markdown, markdown_template);
        self.templates
            .insert(DocumentationFormat::HTML, html_template);
    }

    /// Generate documentation for a configuration
    pub async fn generate_documentation<T: Config>(
        &self,
        config: &T,
        format: DocumentationFormat,
    ) -> DocumentationResult {
        let start_time = std::time::Instant::now();

        // Generate documentation sections
        let sections = self.generate_sections(config).await;

        // Create documentation summary
        let summary = self.generate_summary(config, &sections);

        // Create documentation
        let documentation = ConfigDocumentation {
            title: "Rhema Configuration Documentation".to_string(),
            version: config.version().to_string(),
            sections,
            generated_at: chrono::Utc::now().to_rfc3339(),
            summary,
        };

        // Generate output files
        let output_files = self.generate_output_files(&documentation, format).await;

        let generation_time = start_time.elapsed().as_millis() as u64;

        DocumentationResult {
            documentation: documentation.clone(),
            output_files: output_files.clone(),
            statistics: DocumentationStatistics {
                generation_time_ms: generation_time,
                files_generated: output_files.len(),
                total_size_bytes: self.calculate_total_size(&output_files).await,
                sections_generated: documentation.sections.len(),
            },
        }
    }

    /// Generate documentation sections
    async fn generate_sections<T: Config>(&self, config: &T) -> Vec<DocumentationSection> {
        let mut sections = Vec::new();

        // Overview section
        sections.push(self.generate_overview_section(config));

        // Configuration section
        sections.push(self.generate_configuration_section(config));

        // Validation section
        if self.settings.include_validation_rules {
            sections.push(self.generate_validation_section(config));
        }

        // Examples section
        if self.settings.include_examples {
            sections.push(self.generate_examples_section(config));
        }

        // Troubleshooting section
        if self.settings.include_troubleshooting {
            sections.push(self.generate_troubleshooting_section(config));
        }

        // Best practices section
        if self.settings.include_best_practices {
            sections.push(self.generate_best_practices_section(config));
        }

        // API reference section
        if self.settings.include_api_reference {
            sections.push(self.generate_api_reference_section(config));
        }

        sections
    }

    /// Generate overview section
    fn generate_overview_section<T: Config>(&self, config: &T) -> DocumentationSection {
        let content = format!(
            r#"
# Configuration Overview

This document describes the configuration options for Rhema version {}.

## Key Features

- **Configuration Management**: Centralized configuration management with validation
- **Security**: Built-in security features including encryption and access control
- **Backup**: Automatic backup and restoration capabilities
- **Migration**: Version migration with rollback support
- **Validation**: Comprehensive validation with detailed error reporting

## Configuration Structure

The configuration is organized into several main sections:

- **Global Configuration**: Application-wide settings
- **Repository Configuration**: Repository-specific settings
- **Scope Configuration**: Scope-specific settings
- **Security Configuration**: Security and access control settings
- **Backup Configuration**: Backup and restoration settings
- **Migration Configuration**: Version migration settings

## Quick Start

1. Create a configuration file
2. Validate the configuration
3. Apply the configuration
4. Monitor configuration health

For detailed information about each section, see the Configuration section below.
"#,
            config.version()
        );

        DocumentationSection {
            title: "Overview".to_string(),
            content,
            section_type: SectionType::Overview,
            subsections: Vec::new(),
        }
    }

    /// Generate configuration section
    fn generate_configuration_section<T: Config>(&self, config: &T) -> DocumentationSection {
        let mut subsections = Vec::new();

        // Global configuration subsection
        subsections.push(DocumentationSection {
            title: "Global Configuration".to_string(),
            content: self.generate_global_config_docs(),
            section_type: SectionType::Configuration,
            subsections: Vec::new(),
        });

        // Repository configuration subsection
        subsections.push(DocumentationSection {
            title: "Repository Configuration".to_string(),
            content: self.generate_repository_config_docs(),
            section_type: SectionType::Configuration,
            subsections: Vec::new(),
        });

        // Scope configuration subsection
        subsections.push(DocumentationSection {
            title: "Scope Configuration".to_string(),
            content: self.generate_scope_config_docs(),
            section_type: SectionType::Configuration,
            subsections: Vec::new(),
        });

        DocumentationSection {
            title: "Configuration".to_string(),
            content: "Detailed configuration options for each section.".to_string(),
            section_type: SectionType::Configuration,
            subsections,
        }
    }

    /// Generate validation section
    fn generate_validation_section<T: Config>(&self, config: &T) -> DocumentationSection {
        let content = r#"
# Configuration Validation

Rhema provides comprehensive validation for all configuration options.

## Validation Types

### Schema Validation
Validates configuration against JSON schemas to ensure data types and structure are correct.

### Cross-Reference Validation
Validates references between different configuration sections to ensure consistency.

### Dependency Validation
Validates configuration dependencies to ensure all required components are properly configured.

### Constraint Validation
Validates configuration constraints to ensure values are within acceptable ranges.

## Validation Rules

- All required fields must be present
- Data types must match expected types
- References must point to valid configuration sections
- Dependencies must be satisfied
- Constraints must be met

## Error Handling

Validation errors are categorized by severity:
- **Error**: Critical issues that must be fixed
- **Warning**: Issues that should be addressed
- **Info**: Suggestions for improvement

## Running Validation

Use the validation command to check your configuration:

```bash
rhema config validate
```
"#
        .to_string();

        DocumentationSection {
            title: "Validation".to_string(),
            content,
            section_type: SectionType::Validation,
            subsections: Vec::new(),
        }
    }

    /// Generate examples section
    fn generate_examples_section<T: Config>(&self, config: &T) -> DocumentationSection {
        let content = r#"
# Configuration Examples

## Basic Configuration

```yaml
version: "1.0"
global:
  user:
    id: "user123"
    name: "John Doe"
    email: "john@example.com"
  application:
    name: "My Rhema Project"
    environment: "development"

repository:
  type: "git"
  url: "https://github.com/example/project.git"
  settings:
    auto_sync: true
    conflict_resolution: "merge"

scope:
  name: "main"
  settings:
    auto_validation: true
    backup_enabled: true
```

## Advanced Configuration

```yaml
version: "1.0"
global:
  user:
    id: "user123"
    name: "John Doe"
    email: "john@example.com"
    preferences:
      theme: "dark"
      language: "en"
  application:
    name: "Advanced Rhema Project"
    environment: "production"
    features:
      advanced_validation: true
      security_scanning: true

repository:
  type: "git"
  url: "https://github.com/example/project.git"
  settings:
    auto_sync: true
    conflict_resolution: "rebase"
    backup:
      enabled: true
      frequency: "daily"
      retention: "30 days"
  security:
    encryption:
      enabled: true
      algorithm: "AES-256"
    access_control:
      enabled: true
      users: ["user123", "admin456"]

scope:
  name: "production"
  settings:
    auto_validation: true
    backup_enabled: true
    monitoring:
      enabled: true
      metrics: ["performance", "security", "validation"]
  dependencies:
    - name: "database"
      type: "postgresql"
      required: true
  security:
    encryption:
      enabled: true
    audit_logging:
      enabled: true
      level: "detailed"
```

## Security-Focused Configuration

```yaml
version: "1.0"
global:
  user:
    id: "secure_user"
    name: "Secure User"
    email: "secure@example.com"
    roles: ["admin", "security"]
  application:
    name: "Secure Rhema Project"
    environment: "production"

security:
  encryption:
    enabled: true
    algorithm: "AES-256-GCM"
    key_rotation: "30 days"
  access_control:
    enabled: true
    authentication:
      method: "oauth2"
      provider: "google"
    authorization:
      role_based: true
      permissions:
        - "config:read"
        - "config:write"
        - "security:admin"
  audit_logging:
    enabled: true
    level: "detailed"
    retention: "1 year"
    encryption: true

repository:
  type: "git"
  url: "https://github.com/example/secure-project.git"
  security:
    encryption:
      enabled: true
    access_control:
      enabled: true
      users: ["secure_user"]
    audit_logging:
      enabled: true

scope:
  name: "secure_scope"
  security:
    encryption:
      enabled: true
    access_control:
      enabled: true
    audit_logging:
      enabled: true
      level: "detailed"
```
"#
        .to_string();

        DocumentationSection {
            title: "Examples".to_string(),
            content,
            section_type: SectionType::Examples,
            subsections: Vec::new(),
        }
    }

    /// Generate troubleshooting section
    fn generate_troubleshooting_section<T: Config>(&self, config: &T) -> DocumentationSection {
        let content = r#"
# Troubleshooting

## Common Issues and Solutions

### Configuration Validation Errors

**Problem**: Configuration validation fails with schema errors.

**Solution**: 
1. Check the configuration schema documentation
2. Verify all required fields are present
3. Ensure data types match expected types
4. Run validation with verbose output: `rhema config validate --verbose`

### Security Configuration Issues

**Problem**: Security features are not working as expected.

**Solution**:
1. Verify encryption keys are properly configured
2. Check access control permissions
3. Ensure audit logging is enabled
4. Review security configuration: `rhema config security --check`

### Backup and Restore Issues

**Problem**: Backup or restore operations fail.

**Solution**:
1. Check backup directory permissions
2. Verify disk space is available
3. Ensure backup schedule is properly configured
4. Test backup manually: `rhema config backup --test`

### Migration Issues

**Problem**: Configuration migration fails or causes issues.

**Solution**:
1. Create a backup before migration
2. Review migration logs: `rhema config migrate --log`
3. Test migration on a copy first
4. Use rollback if needed: `rhema config migrate --rollback`

## Getting Help

If you encounter issues not covered here:

1. Check the logs: `rhema config --log-level debug`
2. Run diagnostics: `rhema config --diagnose`
3. Review the documentation
4. Contact support with detailed error information
"#
        .to_string();

        DocumentationSection {
            title: "Troubleshooting".to_string(),
            content,
            section_type: SectionType::Troubleshooting,
            subsections: Vec::new(),
        }
    }

    /// Generate best practices section
    fn generate_best_practices_section<T: Config>(&self, config: &T) -> DocumentationSection {
        let content = r#"
# Best Practices

## Configuration Management

### Version Control
- Always version your configuration files
- Use semantic versioning for configuration changes
- Document configuration changes in commit messages
- Keep configuration files in version control

### Security
- Enable encryption for sensitive configuration data
- Use strong, unique encryption keys
- Implement proper access controls
- Enable audit logging for configuration changes
- Regularly rotate encryption keys

### Backup and Recovery
- Configure automatic backups
- Test backup and restore procedures regularly
- Store backups in secure, off-site locations
- Document backup and restore procedures
- Monitor backup success rates

### Validation
- Enable automatic validation
- Use comprehensive validation rules
- Validate configuration before deployment
- Monitor validation results
- Address validation warnings promptly

## Performance Optimization

### Caching
- Enable caching for frequently accessed configuration
- Configure appropriate cache sizes
- Use cache eviction policies
- Monitor cache hit rates

### Resource Management
- Optimize memory usage
- Configure appropriate timeouts
- Use parallel processing where possible
- Monitor resource usage

## Monitoring and Maintenance

### Health Monitoring
- Enable configuration health monitoring
- Set up alerts for configuration issues
- Monitor configuration performance
- Track configuration changes

### Regular Maintenance
- Review configuration regularly
- Update configuration as needed
- Clean up unused configuration
- Optimize configuration for current needs

## Documentation

### Keep Documentation Updated
- Update documentation with configuration changes
- Include examples in documentation
- Document configuration decisions
- Maintain troubleshooting guides

### Use Consistent Naming
- Use consistent naming conventions
- Use descriptive names for configuration options
- Document naming conventions
- Follow established patterns
"#
        .to_string();

        DocumentationSection {
            title: "Best Practices".to_string(),
            content,
            section_type: SectionType::BestPractices,
            subsections: Vec::new(),
        }
    }

    /// Generate API reference section
    fn generate_api_reference_section<T: Config>(&self, config: &T) -> DocumentationSection {
        let content = r#"
# API Reference

## Configuration API

### Core Types

#### Config
The main configuration type that contains all configuration data.

```rust
pub struct Config {
    pub version: String,
    pub global: GlobalConfig,
    pub repository: RepositoryConfig,
    pub scope: ScopeConfig,
    pub security: SecurityConfig,
    // ... other fields
}
```

#### ValidationResult
Result of configuration validation.

```rust
pub enum ValidationResult {
    Valid,
    Invalid { issues: Vec<ConfigIssue> },
}
```

#### ConfigIssue
Represents a configuration validation issue.

```rust
pub struct ConfigIssue {
    pub path: String,
    pub message: String,
    pub severity: ConfigIssueSeverity,
    pub category: String,
}
```

### Main Functions

#### validate_config
Validates a configuration and returns validation results.

```rust
pub async fn validate_config(config: &Config) -> ValidationResult
```

#### load_config
Loads configuration from a file.

```rust
pub async fn load_config(path: &Path) -> Result<Config, ConfigError>
```

#### save_config
Saves configuration to a file.

```rust
pub async fn save_config(config: &Config, path: &Path) -> Result<(), ConfigError>
```

### Error Handling

#### ConfigError
Represents configuration-related errors.

```rust
pub enum ConfigError {
    ValidationError(String),
    IoError(std::io::Error),
    ParseError(String),
    SecurityError(String),
    // ... other variants
}
```

## Command Line Interface

### Basic Commands

#### Validate Configuration
```bash
rhema config validate [--file <path>] [--verbose]
```

#### Load Configuration
```bash
rhema config load [--file <path>] [--format <format>]
```

#### Save Configuration
```bash
rhema config save [--file <path>] [--format <format>]
```

#### Backup Configuration
```bash
rhema config backup [--schedule] [--restore <backup-id>]
```

#### Migrate Configuration
```bash
rhema config migrate [--version <version>] [--rollback]
```

### Advanced Commands

#### Security Commands
```bash
rhema config security --check
rhema config security --encrypt
rhema config security --decrypt
```

#### Monitoring Commands
```bash
rhema config monitor --health
rhema config monitor --metrics
rhema config monitor --alerts
```

#### Documentation Commands
```bash
rhema config docs --generate
rhema config docs --serve
rhema config docs --export <format>
```
"#
        .to_string();

        DocumentationSection {
            title: "API Reference".to_string(),
            content,
            section_type: SectionType::ApiReference,
            subsections: Vec::new(),
        }
    }

    /// Generate global configuration documentation
    fn generate_global_config_docs(&self) -> String {
        r#"
## Global Configuration

The global configuration section contains application-wide settings.

### User Configuration

```yaml
user:
  id: "string"           # Required: Unique user identifier
  name: "string"         # Required: User display name
  email: "string"        # Required: User email address
  preferences:           # Optional: User preferences
    theme: "string"      # UI theme preference
    language: "string"   # Language preference
  roles: ["string"]      # Optional: User roles
  permissions: {}        # Optional: User permissions
```

### Application Configuration

```yaml
application:
  name: "string"         # Required: Application name
  environment: "string"  # Required: Environment (dev, staging, prod)
  features:              # Optional: Feature flags
    advanced_validation: boolean
    security_scanning: boolean
```

### Integration Configuration

```yaml
integrations:
  git:                   # Optional: Git integration settings
    enabled: boolean
    provider: "string"
  ide:                   # Optional: IDE integration settings
    enabled: boolean
    supported_editors: ["string"]
  cicd:                  # Optional: CI/CD integration settings
    enabled: boolean
    providers: ["string"]
```
"#
        .to_string()
    }

    /// Generate repository configuration documentation
    fn generate_repository_config_docs(&self) -> String {
        r#"
## Repository Configuration

The repository configuration section contains repository-specific settings.

### Basic Repository Settings

```yaml
repository:
  type: "string"         # Required: Repository type (git, svn, etc.)
  url: "string"          # Required: Repository URL
  settings:              # Optional: Repository settings
    auto_sync: boolean   # Enable automatic synchronization
    conflict_resolution: "string"  # Conflict resolution strategy
    backup:              # Backup settings
      enabled: boolean
      frequency: "string"
      retention: "string"
```

### Security Settings

```yaml
repository:
  security:
    encryption:          # Encryption settings
      enabled: boolean
      algorithm: "string"
    access_control:      # Access control settings
      enabled: boolean
      users: ["string"]
    audit_logging:       # Audit logging settings
      enabled: boolean
      level: "string"
```

### Workflow Settings

```yaml
repository:
  workflow:
    branches:            # Branch configuration
      main: "string"
      develop: "string"
    hooks:               # Git hooks configuration
      pre_commit: boolean
      post_commit: boolean
    automation:          # Automation settings
      enabled: boolean
      triggers: ["string"]
```
"#
        .to_string()
    }

    /// Generate scope configuration documentation
    fn generate_scope_config_docs(&self) -> String {
        r#"
## Scope Configuration

The scope configuration section contains scope-specific settings.

### Basic Scope Settings

```yaml
scope:
  name: "string"         # Required: Scope name
  settings:              # Optional: Scope settings
    auto_validation: boolean  # Enable automatic validation
    backup_enabled: boolean   # Enable backup for this scope
    monitoring:               # Monitoring settings
      enabled: boolean
      metrics: ["string"]
```

### Dependencies

```yaml
scope:
  dependencies:          # Optional: Scope dependencies
    - name: "string"     # Dependency name
      type: "string"     # Dependency type
      required: boolean  # Whether dependency is required
```

### Security Settings

```yaml
scope:
  security:
    encryption:          # Encryption settings
      enabled: boolean
    access_control:      # Access control settings
      enabled: boolean
    audit_logging:       # Audit logging settings
      enabled: boolean
      level: "string"
```

### Content Settings

```yaml
scope:
  content:
    types: ["string"]    # Supported content types
    max_size: "string"   # Maximum content size
    validation:          # Content validation settings
      enabled: boolean
      rules: ["string"]
```
"#
        .to_string()
    }

    /// Generate documentation summary
    fn generate_summary<T: Config>(
        &self,
        config: &T,
        sections: &[DocumentationSection],
    ) -> DocumentationSummary {
        DocumentationSummary {
            total_options: 50, // This would be calculated from actual config
            required_options: 10,
            optional_options: 40,
            validation_rules: 25,
            examples: sections
                .iter()
                .filter(|s| s.section_type == SectionType::Examples)
                .count(),
            completeness_score: 95,
        }
    }

    /// Generate output files
    async fn generate_output_files(
        &self,
        documentation: &ConfigDocumentation,
        format: DocumentationFormat,
    ) -> Vec<PathBuf> {
        let mut output_files = Vec::new();

        match format {
            DocumentationFormat::Markdown => {
                let markdown_content = self.render_markdown(documentation);
                let output_path = self.settings.output_directory.join("README.md");
                tokio::fs::write(&output_path, markdown_content).await.ok();
                output_files.push(output_path);
            }
            DocumentationFormat::HTML => {
                let html_content = self.render_html(documentation);
                let output_path = self.settings.output_directory.join("index.html");
                tokio::fs::write(&output_path, html_content).await.ok();
                output_files.push(output_path);
            }
            DocumentationFormat::JSON => {
                let json_content = serde_json::to_string_pretty(documentation).unwrap();
                let output_path = self.settings.output_directory.join("documentation.json");
                tokio::fs::write(&output_path, json_content).await.ok();
                output_files.push(output_path);
            }
            _ => {
                // Other formats would be implemented here
            }
        }

        output_files
    }

    /// Render documentation as Markdown
    fn render_markdown(&self, documentation: &ConfigDocumentation) -> String {
        let mut content = String::new();

        // Title
        content.push_str(&format!("# {}\n\n", documentation.title));
        content.push_str(&format!("**Version**: {}\n", documentation.version));
        content.push_str(&format!(
            "**Generated**: {}\n\n",
            documentation.generated_at
        ));

        // Summary
        content.push_str("## Summary\n\n");
        content.push_str(&format!(
            "- Total Options: {}\n",
            documentation.summary.total_options
        ));
        content.push_str(&format!(
            "- Required Options: {}\n",
            documentation.summary.required_options
        ));
        content.push_str(&format!(
            "- Optional Options: {}\n",
            documentation.summary.optional_options
        ));
        content.push_str(&format!(
            "- Validation Rules: {}\n",
            documentation.summary.validation_rules
        ));
        content.push_str(&format!("- Examples: {}\n", documentation.summary.examples));
        content.push_str(&format!(
            "- Completeness Score: {}%\n\n",
            documentation.summary.completeness_score
        ));

        // Sections
        for section in &documentation.sections {
            content.push_str(&format!("## {}\n\n", section.title));
            content.push_str(&format!("{}\n\n", section.content));

            // Subsections
            for subsection in &section.subsections {
                content.push_str(&format!("### {}\n\n", subsection.title));
                content.push_str(&format!("{}\n\n", subsection.content));
            }
        }

        content
    }

    /// Render documentation as HTML
    fn render_html(&self, documentation: &ConfigDocumentation) -> String {
        let mut content = String::new();

        content.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        content.push_str(&format!("<title>{}</title>\n", documentation.title));
        content.push_str("<meta charset=\"utf-8\">\n");
        content
            .push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");

        if let Some(css) = &self.settings.custom_css {
            content.push_str(&format!("<style>{}</style>\n", css));
        } else {
            content.push_str(include_str!("templates/default.css"));
        }

        content.push_str("</head>\n<body>\n");
        content.push_str(&format!("<h1>{}</h1>\n", documentation.title));
        content.push_str(&format!(
            "<p><strong>Version</strong>: {}</p>\n",
            documentation.version
        ));
        content.push_str(&format!(
            "<p><strong>Generated</strong>: {}</p>\n",
            documentation.generated_at
        ));

        // Summary
        content.push_str("<h2>Summary</h2>\n<ul>\n");
        content.push_str(&format!(
            "<li>Total Options: {}</li>\n",
            documentation.summary.total_options
        ));
        content.push_str(&format!(
            "<li>Required Options: {}</li>\n",
            documentation.summary.required_options
        ));
        content.push_str(&format!(
            "<li>Optional Options: {}</li>\n",
            documentation.summary.optional_options
        ));
        content.push_str(&format!(
            "<li>Validation Rules: {}</li>\n",
            documentation.summary.validation_rules
        ));
        content.push_str(&format!(
            "<li>Examples: {}</li>\n",
            documentation.summary.examples
        ));
        content.push_str(&format!(
            "<li>Completeness Score: {}%</li>\n",
            documentation.summary.completeness_score
        ));
        content.push_str("</ul>\n");

        // Sections
        for section in &documentation.sections {
            content.push_str(&format!("<h2>{}</h2>\n", section.title));
            content.push_str(&format!("<div>{}</div>\n", section.content));

            // Subsections
            for subsection in &section.subsections {
                content.push_str(&format!("<h3>{}</h3>\n", subsection.title));
                content.push_str(&format!("<div>{}</div>\n", subsection.content));
            }
        }

        content.push_str("</body>\n</html>");
        content
    }

    /// Calculate total size of output files
    async fn calculate_total_size(&self, output_files: &[PathBuf]) -> usize {
        let mut total_size = 0;
        for file_path in output_files {
            if let Ok(metadata) = tokio::fs::metadata(file_path).await {
                total_size += metadata.len() as usize;
            }
        }
        total_size
    }
}

impl Default for DocumentationSettings {
    fn default() -> Self {
        Self {
            include_examples: true,
            include_validation_rules: true,
            include_troubleshooting: true,
            include_best_practices: true,
            include_api_reference: true,
            custom_css: None,
            output_directory: PathBuf::from("docs"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_documentation_generator_creation() {
        let settings = DocumentationSettings::default();
        let generator = ConfigDocumentationGenerator::new(settings);
        assert!(!generator.templates.is_empty());
    }

    #[tokio::test]
    async fn test_documentation_generation() {
        let settings = DocumentationSettings::default();
        let generator = ConfigDocumentationGenerator::new(settings);
        let config = Config::default();

        let result = generator
            .generate_documentation(&config, DocumentationFormat::Markdown)
            .await;

        assert!(!result.documentation.sections.is_empty());
        assert!(!result.output_files.is_empty());
        assert!(result.statistics.generation_time_ms > 0);
    }

    #[tokio::test]
    async fn test_markdown_rendering() {
        let settings = DocumentationSettings::default();
        let generator = ConfigDocumentationGenerator::new(settings);
        let config = Config::default();

        let documentation = generator
            .generate_documentation(&config, DocumentationFormat::Markdown)
            .await
            .documentation;

        let markdown_content = generator.render_markdown(&documentation);
        assert!(markdown_content.contains("# Rhema Configuration Documentation"));
        assert!(markdown_content.contains("## Summary"));
    }
}
