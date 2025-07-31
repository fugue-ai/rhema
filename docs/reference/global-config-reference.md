# Global Rhema Configuration Reference

## Overview

The global rhema configuration file is stored at `~/.rhema/config.yaml` and contains user-wide settings that apply across all rhema projects. This file manages personal preferences, application settings, security configurations, and integration settings.

## File Location

```
~/.rhema/config.yaml
```

## Complete Configuration Structure

```yaml
version: "1.0.0"
user:
  id: "user123"
  name: "John Doe"
  email: "john@example.com"
  preferences:
    default_output_format: "yaml"
    default_editor: "vim"
    color_scheme: "dark"
    language: "en"
    timezone: "UTC"
    date_format: "YYYY-MM-DD"
    time_format: "HH:mm:ss"
    notifications:
      email_enabled: true
      desktop_enabled: true
      sound_enabled: false
      frequency: "immediate"
      types: ["errors", "warnings"]
    ui:
      theme: "dark"
      font_size: 14
      show_line_numbers: true
      show_minimap: false
      word_wrap: true
      auto_save: true
      auto_save_interval: 30

application:
  name: "Rhema"
  version: "1.0.0"
  description: "Git-Based Agent Context Protocol CLI"
  settings:
    debug_mode: false
    verbose_logging: false
    log_level: "info"
    log_file: null
    max_log_size: 100
    log_rotation_count: 5
    telemetry_enabled: true
    telemetry_endpoint: "https://telemetry.rhema.dev"
    auto_update_enabled: true
    update_check_interval: 24
  features:
    experimental_features: false
    beta_features: false
    advanced_features: true
    ai_features: true
    cloud_features: false
    collaboration_features: true
    analytics_features: true
  plugins:
    plugin_directory: "~/.rhema/plugins"
    auto_load_plugins: true
    enabled_plugins: ["git", "ai", "analytics"]
    disabled_plugins: []
    plugin_settings: {}

environment:
  current: "development"
  environments:
    development:
      name: "Development"
      description: "Development environment"
      variables:
        DEBUG: "true"
        LOG_LEVEL: "debug"
      paths:
        data: "~/.rhema/data/dev"
        cache: "~/.rhema/cache/dev"
      settings: {}
    testing:
      name: "Testing"
      description: "Testing environment"
      variables:
        DEBUG: "false"
        LOG_LEVEL: "info"
      paths:
        data: "~/.rhema/data/test"
        cache: "~/.rhema/cache/test"
      settings: {}
    staging:
      name: "Staging"
      description: "Staging environment"
      variables:
        DEBUG: "false"
        LOG_LEVEL: "warn"
      paths:
        data: "~/.rhema/data/staging"
        cache: "~/.rhema/cache/staging"
      settings: {}
    production:
      name: "Production"
      description: "Production environment"
      variables:
        DEBUG: "false"
        LOG_LEVEL: "error"
      paths:
        data: "~/.rhema/data/prod"
        cache: "~/.rhema/cache/prod"
      settings: {}
  environment_variables:
    RHEMA_HOME: "~/.rhema"
    RHEMA_CONFIG: "~/.rhema/config.yaml"
  paths:
    home: "~/.rhema"
    config: "~/.rhema"
    data: "~/.rhema/data"
    cache: "~/.rhema/cache"
    log: "~/.rhema/logs"
    temp: "~/.rhema/temp"
    workspace: null
    custom: {}

security:
  encryption:
    enabled: true
    algorithm: "AES-256-GCM"
    key_size: 256
    kdf: "PBKDF2"
    salt_size: 32
    iteration_count: 100000
    key_file: "~/.rhema/keys/master.key"
    master_password_required: true
  authentication:
    method: "password"
    session_timeout: 60
    max_failed_attempts: 5
    lockout_duration: 30
    require_mfa: false
    mfa_method: null
    sso_enabled: false
    sso_provider: null
  authorization:
    rbac_enabled: true
    default_role: "user"
    admin_role: "admin"
    user_role: "user"
    guest_role: "guest"
    permission_inheritance: true
    permission_cache_enabled: true
    permission_cache_timeout: 15
  audit:
    enabled: true
    log_level: "info"
    log_file: "~/.rhema/logs/audit.log"
    retention_days: 90
    events: ["config_change", "auth", "access"]
    filters: {}
  compliance:
    framework: "SOC2"
    level: "Type II"
    reporting_enabled: true
    checks: ["encryption", "access_control", "audit"]
    rules: {}

performance:
  cache:
    enabled: true
    cache_type: "file"
    cache_size: 1000
    cache_ttl: 3600
    cache_directory: "~/.rhema/cache"
    compression_enabled: true
    encryption_enabled: false
  threading:
    max_threads: 8
    thread_pool_size: 4
    async_runtime_threads: 4
    blocking_thread_pool_size: 2
  memory:
    max_memory_usage: 2048
    memory_limit: 4096
    gc_enabled: true
    gc_interval: 300
  network:
    connection_timeout: 30
    request_timeout: 60
    max_connections: 100
    keep_alive_enabled: true
    keep_alive_timeout: 60
    proxy: null

integrations:
  git:
    enabled: true
    provider: "github"
    credentials:
      username: "johndoe"
      email: "john@example.com"
      ssh_key_path: "~/.ssh/id_rsa"
      personal_access_token: null
      oauth_token: null
    hooks_enabled: true
    hooks_directory: null
    workflow: "github-flow"
    branch_naming:
      feature: "feature/{ticket}-{description}"
      bugfix: "bugfix/{ticket}-{description}"
      hotfix: "hotfix/{ticket}-{description}"
  ide:
    enabled: true
    supported_ides: ["vscode", "intellij", "vim"]
    ide_settings:
      vscode:
        auto_sync: true
        extensions: ["rhema-vscode"]
      intellij:
        auto_sync: true
        plugins: ["rhema-intellij"]
    auto_sync_enabled: true
    sync_interval: 30
  cicd:
    enabled: false
    provider: "github-actions"
    settings: {}
    auto_deploy_enabled: false
    deployment_environments: []
  cloud:
    enabled: false
    provider: "aws"
    credentials:
      access_key_id: null
      secret_access_key: null
      session_token: null
      credentials_file: null
      profile_name: null
    regions: ["us-east-1"]
    services: ["s3", "ec2"]
  external_services: {}

custom: {}
audit_log:
  changes: []
  created_at: "2024-01-01T00:00:00Z"
  updated_at: "2024-01-01T00:00:00Z"
health:
  status: "healthy"
  issues: []
  recommendations: []
  last_check: "2024-01-01T00:00:00Z"
stats:
  total_configs: 1
  global_configs: 1
  repository_configs: 0
  scope_configs: 0
  encrypted_configs: 1
  backup_count: 0
  last_backup: null
  validation_errors: 0
  migration_pending: 0
updated_at: "2024-01-01T00:00:00Z"
```

## Configuration Sections

### 1. User Section

The `user` section contains personal information and preferences:

- **id**: Unique user identifier
- **name**: User's full name
- **email**: User's email address
- **preferences**: User-specific settings including:
  - Output format preferences
  - Editor settings
  - UI theme and appearance
  - Notification settings
  - Language and timezone preferences

### 2. Application Section

The `application` section manages core application behavior:

- **name**: Application name
- **version**: Current version
- **settings**: Core application settings including:
  - Debug and logging configuration
  - Telemetry settings
  - Auto-update preferences
- **features**: Feature flags for enabling/disabling functionality
- **plugins**: Plugin management settings

### 3. Environment Section

The `environment` section handles environment-specific configurations:

- **current**: Currently active environment
- **environments**: Environment-specific settings for development, testing, staging, and production
- **environment_variables**: Global environment variables
- **paths**: Directory paths for various data types

### 4. Security Section

The `security` section manages all security-related configurations:

- **encryption**: Encryption algorithm and key management
- **authentication**: Authentication methods and session management
- **authorization**: Role-based access control settings
- **audit**: Audit logging configuration
- **compliance**: Compliance framework settings

### 5. Performance Section

The `performance` section optimizes application performance:

- **cache**: Cache configuration and settings
- **threading**: Thread pool and concurrency settings
- **memory**: Memory management and garbage collection
- **network**: Network connection and timeout settings

### 6. Integrations Section

The `integrations` section configures external service integrations:

- **git**: Git provider and workflow settings
- **ide**: IDE integration settings
- **cicd**: CI/CD pipeline configuration
- **cloud**: Cloud service provider settings
- **external_services**: Additional external service configurations

### 7. System Sections

- **custom**: Custom configuration extensions
- **audit_log**: Configuration change tracking
- **health**: System health status and recommendations
- **stats**: Configuration statistics and metrics

## CLI Commands for Configuration Management

```bash
# Display global configuration
rhema config show global [--format <format>]

# Edit global configuration
rhema config edit global [--editor <editor>]

# Validate global configuration
rhema config validate global [--fix]

# Check configuration health
rhema config health global

# Backup global configuration
rhema config backup global

# Export global configuration
rhema config export global --format yaml --output global-config.yaml

# Import global configuration
rhema config import global --format yaml --input global-config.yaml
```

## Configuration Hierarchy

The rhema configuration system follows this hierarchy:

1. **Global Configuration** (`~/.rhema/config.yaml`) - User-wide settings
2. **Repository Configuration** (`.rhema/config.yaml`) - Project-specific settings
3. **Scope Configuration** - Scope-specific settings within projects

Global configuration provides defaults that can be overridden by repository-specific configurations.

## Best Practices

1. **Backup Regularly**: Use `rhema config backup global` to create regular backups
2. **Validate Changes**: Always validate configuration after making changes
3. **Use Version Control**: Consider versioning your global configuration
4. **Secure Sensitive Data**: Ensure encryption is enabled for sensitive configurations
5. **Environment Separation**: Use different configurations for different environments

## Troubleshooting

### Common Issues

1. **Configuration Not Found**: Ensure the file exists at `~/.rhema/config.yaml`
2. **Permission Errors**: Check file permissions and ownership
3. **Validation Errors**: Use `rhema config validate global --fix` to auto-fix issues
4. **Performance Issues**: Review and adjust performance settings

### Health Checks

```bash
# Check configuration health
rhema config health global

# View health recommendations
rhema config health global --verbose
```

## Migration and Updates

When updating rhema versions, the configuration system automatically handles migrations:

```bash
# Check for pending migrations
rhema config migrate global --dry-run

# Apply migrations
rhema config migrate global
```

## Related Documentation

- [Configuration Management](./configuration-management.md)
- [Repository Configuration](./repository-config-reference.md)
- [CLI Command Reference](./cli-command-reference.md)
- [Security Configuration](./security-configuration.md) 