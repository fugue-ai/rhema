# Rhema Integrations Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-integrations)](https://crates.io/crates/rhema-integrations)
[![Documentation](https://docs.rs/rhema-integrations/badge.svg)](https://docs.rs/rhema-integrations)

External integrations for Rhema, providing connectivity to project management tools, communication platforms, development services, analytics platforms, and documentation systems.

## Overview

The `rhema-integrations` crate provides a comprehensive framework for integrating Rhema with external services and tools. It offers a unified interface for connecting with project management platforms, communication tools, development services, analytics platforms, and documentation systems. The crate is designed with a modular architecture that makes it easy to add new integrations and manage existing ones.

## Features

### 📋 Project Management Integrations
- **Jira Integration**: Create, read, update, and search issues
- **Asana Integration**: Task and project management with workspace support
- **Trello Integration**: Card and board management
- **GitHub Issues Integration**: Issue tracking and repository management
- **GitLab Issues Integration**: Issue management for GitLab projects

### 💬 Communication Integrations
- **Slack Integration**: Send messages, blocks, and manage channels
- **Discord Integration**: Message sending and webhook support
- **Microsoft Teams Integration**: Webhook messaging and adaptive cards
- **Email Integration**: SMTP-based email notifications with HTML support

### 🔧 Development Tool Integrations
- **IDE Integration**: Open files and projects in various IDEs (VS Code, Vim, Sublime)
- **Code Review Integration**: Pull request management and commenting
- **Testing Integration**: Test execution and coverage reporting
- **Build Integration**: Build system management and status tracking
- **Deployment Integration**: Environment deployment and rollback management

### 📊 Analytics & Monitoring Integrations
- **Analytics Integration**: Event tracking and user identification
- **Monitoring Integration**: Metrics, logs, and alert management
- **Logging Integration**: Log management and search capabilities
- **Performance Integration**: Performance metrics and tracing
- **Business Intelligence Integration**: Query execution and dashboard management

### 📚 Documentation Integrations
- **Confluence Integration**: Page creation, updates, and search
- **Notion Integration**: Page and block management
- **ReadTheDocs Integration**: Documentation project management
- **Wiki Integration**: Generic wiki page management

### 🔄 Core Framework Features
- **Unified Integration Interface**: Common trait for all integrations
- **HTTP Client Management**: Centralized HTTP client with retry logic
- **Configuration Management**: Flexible configuration system
- **Status Monitoring**: Connection status and health checks
- **Error Handling**: Comprehensive error handling and reporting

## Architecture

```
rhema-integrations/
├── integrations.rs      # Core integration framework and traits
├── project_management.rs # Project management integrations
│   ├── JiraIntegration
│   ├── AsanaIntegration
│   ├── TrelloIntegration
│   ├── GitHubIssuesIntegration
│   └── GitLabIssuesIntegration
├── communication.rs     # Communication platform integrations
│   ├── SlackIntegration
│   ├── DiscordIntegration
│   ├── MicrosoftTeamsIntegration
│   └── EmailIntegration
├── development.rs       # Development tool integrations
│   ├── IDEIntegration
│   ├── CodeReviewIntegration
│   ├── TestingIntegration
│   ├── BuildIntegration
│   └── DeploymentIntegration
├── analytics.rs         # Analytics and monitoring integrations
│   ├── AnalyticsIntegration
│   ├── MonitoringIntegration
│   ├── LoggingIntegration
│   ├── PerformanceIntegration
│   └── BusinessIntelligenceIntegration
├── documentation.rs     # Documentation platform integrations
│   ├── ConfluenceIntegration
│   ├── NotionIntegration
│   ├── ReadTheDocsIntegration
│   └── WikiIntegration
└── lib.rs              # Library entry point
```

## Usage

### Basic Integration Setup

```rust
use rhema_integrations::{
    IntegrationManager, IntegrationConfig, IntegrationType,
    JiraIntegration, SlackIntegration
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create integration manager
    let mut manager = IntegrationManager::new();
    
    // Configure Jira integration
    let jira_config = IntegrationConfig {
        name: "jira".to_string(),
        integration_type: IntegrationType::Jira,
        base_url: Some("https://company.atlassian.net".to_string()),
        api_key: Some("your-api-key".to_string()),
        enabled: true,
        ..Default::default()
    };
    
    // Configure Slack integration
    let slack_config = IntegrationConfig {
        name: "slack".to_string(),
        integration_type: IntegrationType::Slack,
        token: Some("your-slack-token".to_string()),
        enabled: true,
        ..Default::default()
    };
    
    // Initialize integrations
    manager.initialize_from_config(vec![jira_config, slack_config]).await?;
    
    Ok(())
}
```

### Project Management Integration

```rust
use rhema_integrations::project_management::JiraIntegration;

// Create Jira integration
let mut jira = JiraIntegration::new();
jira.initialize(IntegrationConfig {
    name: "jira".to_string(),
    integration_type: IntegrationType::Jira,
    base_url: Some("https://company.atlassian.net".to_string()),
    token: Some("your-jira-token".to_string()),
    enabled: true,
    ..Default::default()
}).await?;

// Create Jira issue
let issue_key = jira.create_issue(
    "PROJ",
    "Implement JWT authentication",
    "Add JWT-based authentication system to the API",
    "Task"
).await?;

// Get issue details
let issue = jira.get_issue(&issue_key).await?;

// Search issues
let issues = jira.search_issues("project = PROJ AND status = 'In Progress'", Some(50)).await?;
```

### Communication Integration

```rust
use rhema_integrations::communication::SlackIntegration;

// Create Slack integration
let mut slack = SlackIntegration::new();
slack.initialize(IntegrationConfig {
    name: "slack".to_string(),
    integration_type: IntegrationType::Slack,
    token: Some("your-slack-token".to_string()),
    enabled: true,
    ..Default::default()
}).await?;

// Send simple message
let timestamp = slack.send_message(
    "#development",
    "New todo created: Implement JWT authentication",
    None
).await?;

// Send message with blocks (rich formatting)
let blocks = vec![
    serde_json::json!({
        "type": "section",
        "text": {
            "type": "mrkdwn",
            "text": "*New High Priority Todo*"
        }
    }),
    serde_json::json!({
        "type": "section",
        "fields": [
            {
                "type": "mrkdwn",
                "text": "*Priority:* High"
            },
            {
                "type": "mrkdwn",
                "text": "*Status:* Pending"
            }
        ]
    })
];

slack.send_blocks("#alerts", blocks).await?;
```

### Development Tool Integration

```rust
use rhema_integrations::development::IDEIntegration;

// Create IDE integration
let mut ide = IDEIntegration::new();
ide.initialize(IntegrationConfig {
    name: "vscode".to_string(),
    integration_type: IntegrationType::IDE,
    custom_headers: {
        let mut headers = HashMap::new();
        headers.insert("ide_command".to_string(), "code".to_string());
        headers
    },
    enabled: true,
    ..Default::default()
}).await?;

// Open file at specific line
ide.open_file("src/main.rs", Some(42)).await?;

// Open project
ide.open_project("/path/to/project").await?;
```

### Analytics Integration

```rust
use rhema_integrations::analytics::AnalyticsIntegration;

// Create analytics integration
let mut analytics = AnalyticsIntegration::new();
analytics.initialize(IntegrationConfig {
    name: "analytics".to_string(),
    integration_type: IntegrationType::Analytics,
    base_url: Some("https://analytics.company.com".to_string()),
    api_key: Some("your-analytics-key".to_string()),
    enabled: true,
    ..Default::default()
}).await?;

// Track event
let mut properties = HashMap::new();
properties.insert("todo_id".to_string(), serde_json::Value::String("123".to_string()));
properties.insert("priority".to_string(), serde_json::Value::String("high".to_string()));

analytics.track_event("todo_created", properties).await?;

// Identify user
let mut traits = HashMap::new();
traits.insert("email".to_string(), serde_json::Value::String("user@company.com".to_string()));
traits.insert("role".to_string(), serde_json::Value::String("developer".to_string()));

analytics.identify_user("user123", traits).await?;
```

### Documentation Integration

```rust
use rhema_integrations::documentation::ConfluenceIntegration;

// Create Confluence integration
let mut confluence = ConfluenceIntegration::new();
confluence.initialize(IntegrationConfig {
    name: "confluence".to_string(),
    integration_type: IntegrationType::Confluence,
    base_url: Some("https://company.atlassian.net/wiki".to_string()),
    token: Some("your-confluence-token".to_string()),
    enabled: true,
    ..Default::default()
}).await?;

// Create documentation page
let page_id = confluence.create_page(
    "DEV",
    "JWT Authentication Implementation",
    "# JWT Authentication\n\nThis document describes the JWT authentication implementation...",
    None
).await?;

// Update existing page
confluence.update_page(&page_id, "Updated Title", "Updated content...", 2).await?;
```

## Configuration

### Integration Configuration Structure

```yaml
# .rhema/integrations.yaml
integrations:
  project_management:
    jira:
      enabled: true
      base_url: "https://company.atlassian.net"
      token: "${JIRA_API_TOKEN}"
      timeout_seconds: 30
      retry_attempts: 3
    
    github_issues:
      enabled: true
      token: "${GITHUB_TOKEN}"
      base_url: "https://api.github.com"
  
  communication:
    slack:
      enabled: true
      token: "${SLACK_BOT_TOKEN}"
      custom_headers:
        default_channel: "#general"
    
    email:
      enabled: true
      base_url: "smtp://smtp.company.com:587"
      username: "${EMAIL_USERNAME}"
      password: "${EMAIL_PASSWORD}"
  
  development:
    ide:
      enabled: true
      custom_headers:
        ide_command: "code"
    
    testing:
      enabled: true
      custom_headers:
        test_command: "cargo test"
  
  analytics:
    analytics:
      enabled: true
      base_url: "https://analytics.company.com"
      api_key: "${ANALYTICS_API_KEY}"
    
    monitoring:
      enabled: true
      base_url: "https://monitoring.company.com"
      api_key: "${MONITORING_API_KEY}"
  
  documentation:
    confluence:
      enabled: true
      base_url: "https://company.atlassian.net/wiki"
      token: "${CONFLUENCE_TOKEN}"
```

### Environment Variables

The integration system supports environment variable substitution for sensitive configuration values:

```bash
# Project Management
export JIRA_API_TOKEN="your-jira-token"
export GITHUB_TOKEN="your-github-token"

# Communication
export SLACK_BOT_TOKEN="your-slack-token"
export EMAIL_USERNAME="your-email"
export EMAIL_PASSWORD="your-password"

# Analytics & Monitoring
export ANALYTICS_API_KEY="your-analytics-key"
export MONITORING_API_KEY="your-monitoring-key"

# Documentation
export CONFLUENCE_TOKEN="your-confluence-token"
```

## Dependencies

- **rhema-core**: Core Rhema functionality and error types
- **reqwest**: HTTP client for API calls
- **tokio**: Async runtime for concurrent operations
- **serde**: Serialization support for configuration and data
- **serde_json**: JSON serialization and deserialization
- **chrono**: Date and time handling
- **async-trait**: Async trait support
- **clap**: Command-line argument parsing
- **anyhow**: Error handling utilities

## Development Status

### ✅ Implemented Features
- **Core Framework**: Integration manager, configuration system, HTTP client
- **Project Management**: Jira, Asana, Trello, GitHub Issues, GitLab Issues
- **Communication**: Slack, Discord, Microsoft Teams, Email
- **Development Tools**: IDE, Code Review, Testing, Build, Deployment
- **Analytics & Monitoring**: Analytics, Monitoring, Logging, Performance, BI
- **Documentation**: Confluence, Notion, ReadTheDocs, Wiki
- **Authentication**: Token-based and basic authentication support
- **Error Handling**: Comprehensive error types and handling
- **Status Monitoring**: Connection health checks and status reporting

### 🔄 In Progress
- Integration testing framework
- Performance optimizations
- Advanced configuration validation
- Plugin system for custom integrations

### 📋 Planned Features
- Real-time synchronization capabilities
- Advanced conflict resolution
- Integration marketplace
- Enterprise features (SSO, advanced security)
- Webhook support for bidirectional communication
- Integration metrics and analytics

## Testing

The integrations crate includes comprehensive testing support:

```bash
# Run all tests
cargo test

# Run specific integration tests
cargo test --test jira_integration
cargo test --test slack_integration

# Run with integration coverage
cargo test --features test-integrations
```

## Contributing

1. Check the current implementation status in the source code
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all integrations implement the `ExternalIntegration` trait
4. Add comprehensive error handling and status reporting
5. Include proper documentation and examples
6. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 