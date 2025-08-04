# Rhema Integrations Crate

[![Crates.io](https://img.shields.io/crates/v/rhema-integrations)](https://crates.io/crates/rhema-integrations)
[![Documentation](https://docs.rs/rhema-integrations/badge.svg)](https://docs.rs/rhema-integrations)

External integrations for Rhema, providing connectivity to project management tools, communication platforms, and development services.

## Overview

The `rhema-integrations` crate provides external integrations for Rhema, enabling connectivity with project management tools, communication platforms, and development services. It extends Rhema's capabilities by integrating with the broader development ecosystem.

## Features

### 📋 Project Management Integrations
- **Jira Integration**: Connect with Jira for issue tracking and project management
- **GitHub Integration**: Integrate with GitHub for repository management and workflows
- **GitLab Integration**: Connect with GitLab for CI/CD and project management
- **Linear Integration**: Integrate with Linear for modern project management
- **Asana Integration**: Connect with Asana for task and project management

### 💬 Communication Integrations
- **Slack Integration**: Integrate with Slack for notifications and team communication
- **Discord Integration**: Connect with Discord for community and team chat
- **Microsoft Teams Integration**: Integrate with Teams for enterprise communication
- **Email Integration**: Email notifications and communication
- **Webhook Integration**: Generic webhook support for custom integrations

### 🔧 Development Tool Integrations
- **IDE Integrations**: Integration with popular IDEs and editors
- **CI/CD Integrations**: Connect with CI/CD pipelines and automation
- **Monitoring Integrations**: Integrate with monitoring and observability tools
- **Testing Integrations**: Connect with testing frameworks and tools
- **Documentation Integrations**: Integrate with documentation platforms

### 🔄 Data Synchronization
- **Bidirectional Sync**: Bidirectional data synchronization with external tools
- **Real-time Updates**: Real-time updates and notifications
- **Conflict Resolution**: Handle conflicts between Rhema and external data
- **Data Validation**: Validate data integrity across integrations

### 🔐 Authentication and Security
- **OAuth Support**: OAuth authentication for external services
- **API Key Management**: Secure API key management
- **Token Refresh**: Automatic token refresh and renewal
- **Access Control**: Role-based access control for integrations

## Architecture

```
rhema-integrations/
├── project_management/  # Project management integrations
│   ├── jira.rs         # Jira integration
│   ├── github.rs       # GitHub integration
│   ├── gitlab.rs       # GitLab integration
│   ├── linear.rs       # Linear integration
│   └── asana.rs        # Asana integration
├── communication/       # Communication integrations
│   ├── slack.rs        # Slack integration
│   ├── discord.rs      # Discord integration
│   ├── teams.rs        # Microsoft Teams integration
│   ├── email.rs        # Email integration
│   └── webhook.rs      # Webhook integration
├── development/         # Development tool integrations
│   ├── ide.rs          # IDE integrations
│   ├── cicd.rs         # CI/CD integrations
│   ├── monitoring.rs   # Monitoring integrations
│   ├── testing.rs      # Testing integrations
│   └── documentation.rs # Documentation integrations
├── sync.rs             # Data synchronization
├── auth.rs             # Authentication and security
└── lib.rs              # Library entry point
```

## Usage

### Project Management Integration

```rust
use rhema_integrations::project_management::jira::JiraIntegration;
use rhema_integrations::project_management::github::GitHubIntegration;

// Jira integration
let jira = JiraIntegration::new("https://company.atlassian.net", &api_token)?;

// Create Jira issue from Rhema todo
let issue = jira.create_issue(CreateIssueRequest {
    project_key: "PROJ".to_string(),
    summary: "Implement JWT authentication".to_string(),
    description: "Add JWT-based authentication system".to_string(),
    issue_type: "Task".to_string(),
})?;

// GitHub integration
let github = GitHubIntegration::new(&github_token)?;

// Create GitHub issue
let github_issue = github.create_issue("owner/repo", CreateIssueRequest {
    title: "Implement JWT authentication".to_string(),
    body: "Add JWT-based authentication system".to_string(),
    labels: vec!["enhancement".to_string()],
})?;
```

### Communication Integration

```rust
use rhema_integrations::communication::slack::SlackIntegration;
use rhema_integrations::communication::email::EmailIntegration;

// Slack integration
let slack = SlackIntegration::new(&webhook_url)?;

// Send notification to Slack
slack.send_message(SlackMessage {
    channel: "#development".to_string(),
    text: "New todo created: Implement JWT authentication".to_string(),
    attachments: vec![
        SlackAttachment {
            title: "Todo Details".to_string(),
            text: "Priority: High, Status: Pending".to_string(),
            color: "warning".to_string(),
        }
    ],
})?;

// Email integration
let email = EmailIntegration::new(smtp_config)?;

// Send email notification
email.send_notification(EmailNotification {
    to: vec!["team@company.com".to_string()],
    subject: "New High Priority Todo".to_string(),
    body: "A new high priority todo has been created.".to_string(),
})?;
```

### Development Tool Integration

```rust
use rhema_integrations::development::cicd::CICDIntegration;
use rhema_integrations::development::monitoring::MonitoringIntegration;

// CI/CD integration
let cicd = CICDIntegration::new(&jenkins_url, &jenkins_token)?;

// Trigger build
cicd.trigger_build("user-service-pipeline", BuildParameters {
    branch: "feature/jwt-auth".to_string(),
    parameters: HashMap::new(),
})?;

// Monitoring integration
let monitoring = MonitoringIntegration::new(&datadog_api_key)?;

// Send custom metric
monitoring.send_metric("rhema.todos.created", 1.0, vec![
    ("scope".to_string(), "user-service".to_string()),
    ("priority".to_string(), "high".to_string()),
])?;
```

### Data Synchronization

```rust
use rhema_integrations::sync::SyncManager;

let sync_manager = SyncManager::new();

// Configure bidirectional sync
sync_manager.configure_sync(SyncConfig {
    source: "rhema".to_string(),
    target: "jira".to_string(),
    direction: SyncDirection::Bidirectional,
    mapping: SyncMapping {
        todo_to_issue: true,
        issue_to_todo: true,
        status_mapping: HashMap::new(),
    },
})?;

// Start synchronization
sync_manager.start_sync()?;

// Handle conflicts
sync_manager.on_conflict(|conflict| {
    println!("Conflict detected: {:?}", conflict);
    // Resolve conflict
    ConflictResolution::UseRhema
});
```

## Configuration

### Integration Configuration

```yaml
# .rhema/integrations.yaml
integrations:
  project_management:
    jira:
      enabled: true
      url: "https://company.atlassian.net"
      api_token: "${JIRA_API_TOKEN}"
      project_mapping:
        "user-service": "PROJ"
    
    github:
      enabled: true
      token: "${GITHUB_TOKEN}"
      repositories:
        - "owner/repo"
    
    linear:
      enabled: true
      api_key: "${LINEAR_API_KEY}"
      team_id: "team-id"
  
  communication:
    slack:
      enabled: true
      webhook_url: "${SLACK_WEBHOOK_URL}"
      channels:
        - "#development"
        - "#alerts"
    
    email:
      enabled: true
      smtp:
        host: "smtp.company.com"
        port: 587
        username: "${EMAIL_USERNAME}"
        password: "${EMAIL_PASSWORD}"
  
  development:
    cicd:
      jenkins:
        enabled: true
        url: "https://jenkins.company.com"
        token: "${JENKINS_TOKEN}"
      
      github_actions:
        enabled: true
        token: "${GITHUB_TOKEN}"
    
    monitoring:
      datadog:
        enabled: true
        api_key: "${DATADOG_API_KEY}"
        app_key: "${DATADOG_APP_KEY}"
```

### Sync Configuration

```yaml
integrations:
  sync:
    bidirectional:
      enabled: true
      interval: 5m
      conflict_resolution: "rhema_priority"
    
    mappings:
      jira:
        todo_to_issue: true
        issue_to_todo: true
        status_mapping:
          "pending": "To Do"
          "in_progress": "In Progress"
          "completed": "Done"
      
      github:
        todo_to_issue: true
        issue_to_todo: false
        label_mapping:
          "high_priority": "priority:high"
          "bug": "type:bug"
```

## Dependencies

- **rhema-core**: Core Rhema functionality
- **reqwest**: HTTP client for API calls
- **serde**: Serialization support
- **tokio**: Async runtime
- **oauth2**: OAuth authentication
- **lettre**: Email functionality
- **slack-hook**: Slack webhook support

## Development Status

### ✅ Completed Features
- Basic integration framework
- Authentication infrastructure
- Data synchronization framework
- Webhook support

### 🔄 In Progress
- Project management integrations
- Communication integrations
- Development tool integrations
- Advanced sync features

### 📋 Planned Features
- Advanced authentication
- Real-time synchronization
- Plugin system
- Enterprise features

## Contributing

1. Check the [TODO.md](./TODO.md) for current priorities
2. Follow the [Rhema contribution guidelines](../../CONTRIBUTING.md)
3. Ensure all integrations are properly tested
4. Run the test suite: `cargo test`

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../../LICENSE) for details. 