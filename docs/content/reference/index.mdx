# API Reference

This section provides comprehensive technical reference documentation for Rhema, including schemas, APIs, configuration options, and implementation details.

## 🎯 Overview

The API Reference serves as the definitive technical documentation for Rhema developers, integrators, and advanced users. It provides detailed specifications for all components, interfaces, and data structures.

## 📚 Reference Categories

### 📋 Schemas and Specifications
- **[Global Config Reference](./global-config-reference.md)** - Complete configuration schema reference
- **[Specification Schema Examples](./specification-schema-examples.md)** - Example schemas and specifications
- **[YAML Schema Reference](./yaml-schema-reference.md)** - All YAML file schemas
- **[JSON Schema Definitions](./json-schema-definitions.md)** - JSON schema specifications

### 🔧 Core APIs
- **[CLI API Reference](./cli-api-reference.md)** - Command-line interface specifications
- **[Library API Reference](./library-api-reference.md)** - Rust library API documentation
- **[Query Language Reference](./query-language-reference.md)** - CQL syntax and semantics
- **[Plugin API Reference](./plugin-api-reference.md)** - Plugin development interfaces

### 🌐 Integration APIs
- **[MCP Protocol Reference](./mcp-protocol-reference.md)** - Model Context Protocol implementation
- **[Action Protocol Reference](./action-protocol-reference.md)** - Action protocol specifications
- **[gRPC API Reference](./grpc-api-reference.md)** - gRPC service definitions
- **[REST API Reference](./rest-api-reference.md)** - REST API endpoints (if applicable)

### 🔒 Security and Authentication
- **[Security Model](./security-model.md)** - Security architecture and model
- **[Authentication Reference](./authentication-reference.md)** - Authentication mechanisms
- **[Authorization Reference](./authorization-reference.md)** - Authorization and permissions
- **[Encryption Reference](./encryption-reference.md)** - Data encryption specifications

### 📊 Data Models
- **[Data Model Reference](./data-model-reference.md)** - Core data structures
- **[Context Model Reference](./context-model-reference.md)** - Context data models
- **[Knowledge Model Reference](./knowledge-model-reference.md)** - Knowledge representation
- **[Agent Model Reference](./agent-model-reference.md)** - Agent coordination models

## 🚀 Quick Reference

### Essential Commands
```bash
# Core operations
rhema init [--scope-type TYPE] [--scope-name NAME]
rhema query "CQL_QUERY" [--format FORMAT]
rhema validate [--recursive] [--strict]
rhema health [--scope SCOPE]

# Knowledge management
rhema todo add "TITLE" [--priority LEVEL]
rhema insight record "INSIGHT" [--confidence LEVEL]
rhema decision record "TITLE" [--status STATUS]

# Advanced operations
rhema dependencies [--visualize] [--conflicts]
rhema export-context [--format FORMAT] [--ai-agent-format]
rhema bootstrap-context [--use-case USE_CASE]
```

### Configuration Structure
```yaml
# Global configuration
rhema:
  version: "1.0.0"
  config:
    validation:
      strict: false
      recursive: true
    ai_integration:
      enabled: true
      context_injection: true
    performance:
      cache_enabled: true
      cache_ttl: 3600

# Scope configuration
name: "service-name"
type: "service"
dependencies:
  - name: "dependency-name"
    version: "1.0.0"
config:
  validation:
    strict: true
```

### Data Schemas
```yaml
# Todo schema
tasks:
  - id: "TASK-001"
    title: "Task title"
    priority: "high"  # low, medium, high, critical
    status: "todo"    # todo, in_progress, review, done, cancelled
    tags: ["tag1", "tag2"]

# Insight schema
insights:
  - id: "INSIGHT-001"
    insight: "Insight description"
    confidence: "high"  # low, medium, high
    category: "performance"  # performance, security, architecture, process, other

# Decision schema
decisions:
  - id: "DECISION-001"
    title: "Decision title"
    status: "approved"  # proposed, approved, rejected, implemented, deprecated
    decision_type: "architecture"  # architecture, technology, process, business, other
```

## 🔍 Query Language Reference

### CQL Syntax
```sql
-- Basic queries
find all todos
find insights where confidence = high
find decisions where status = approved

-- Complex queries
find todos where priority = high AND status = todo
find insights containing 'performance' OR tags contains 'optimization'
find decisions where metadata.team = platform AND created_date > 2024-01-01

-- Aggregations
count todos by status
group insights by confidence, tags
count decisions by status, metadata.team

-- Cross-scope queries
find todos in services where priority = high
find insights in libraries containing 'performance'
```

### Query Operators
- **Comparison**: `=`, `!=`, `>`, `<`, `>=`, `<=`
- **Logical**: `AND`, `OR`, `NOT`
- **String**: `contains`, `starts with`, `ends with`, `matches`
- **Array**: `contains`, `not contains`, `any in`, `all in`
- **Null**: `is null`, `is not null`

## 🔧 Configuration Reference

### Global Configuration Options
```yaml
# .rhema/config.yaml
rhema:
  # Validation settings
  validation:
    strict: false              # Treat warnings as errors
    recursive: true            # Validate subdirectories
    auto_fix: false            # Automatically fix issues
    include_warnings: true     # Include warnings in output
    custom_rules: null         # Path to custom validation rules
  
  # AI integration settings
  ai_integration:
    enabled: true              # Enable AI features
    context_injection: true    # Inject context into AI
    prompt_optimization: true  # Optimize prompts
    model_provider: "openai"   # AI model provider
  
  # Performance settings
  performance:
    cache_enabled: true        # Enable caching
    cache_ttl: 3600           # Cache TTL in seconds
    query_optimization: true   # Optimize queries
    parallel_processing: true  # Enable parallel processing
  
  # Monitoring settings
  monitoring:
    enabled: true              # Enable monitoring
    metrics_collection: true   # Collect metrics
    health_checks: true        # Enable health checks
    alerting: false            # Enable alerting
  
  # Security settings
  security:
    encryption_enabled: false  # Enable data encryption
    audit_logging: true        # Enable audit logging
    access_control: false      # Enable access control
```

### Scope Configuration Options
```yaml
# scope.yaml
name: "service-name"
type: "service"  # service, app, library, component, infrastructure, documentation, tool
description: "Service description"
version: "1.0.0"

# Metadata
metadata:
  team: "team-name"
  repository: "github.com/org/repo"
  language: "rust"
  framework: "actix-web"

# Dependencies
dependencies:
  - name: "dependency-name"
    version: "1.0.0"
    type: "library"
    relationship: "uses"

# Configuration
config:
  validation:
    strict: true
    recursive: true
    custom_rules: "scope-validation.yaml"
  
  ai_integration:
    enabled: true
    context_injection: true
  
  performance:
    cache_enabled: true
    cache_ttl: 1800
```

## 📊 Data Model Reference

### Core Data Types
```rust
// Todo item
struct Todo {
    id: String,
    title: String,
    description: Option<String>,
    priority: Priority,  // low, medium, high, critical
    status: Status,      // todo, in_progress, review, done, cancelled
    tags: Vec<String>,
    created_date: DateTime<Utc>,
    due_date: Option<DateTime<Utc>>,
    assignee: Option<String>,
    context: Context,
    acceptance_criteria: Vec<String>,
}

// Insight
struct Insight {
    id: String,
    insight: String,
    confidence: Confidence,  // low, medium, high
    category: Category,      // performance, security, architecture, process, other
    tags: Vec<String>,
    created_date: DateTime<Utc>,
    context: Context,
    references: Vec<Reference>,
}

// Decision
struct Decision {
    id: String,
    title: String,
    description: Option<String>,
    status: DecisionStatus,  // proposed, approved, rejected, implemented, deprecated
    decision_type: DecisionType,  // architecture, technology, process, business, other
    tags: Vec<String>,
    created_date: DateTime<Utc>,
    decided_date: Option<DateTime<Utc>>,
    context: DecisionContext,
    stakeholders: Vec<Stakeholder>,
    implementation: Option<Implementation>,
}
```

### Context Models
```rust
// General context
struct Context {
    related_files: Vec<String>,
    dependencies: Vec<String>,
    related_items: Vec<RelatedItem>,
    metadata: HashMap<String, Value>,
}

// Decision context
struct DecisionContext {
    problem: String,
    alternatives: Vec<Alternative>,
    criteria: Vec<String>,
    outcome: String,
    risks: Vec<String>,
    benefits: Vec<String>,
}
```

## 🔌 Plugin API Reference

### Plugin Interface
```rust
// Plugin trait
trait RhemaPlugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    
    fn initialize(&mut self, config: &PluginConfig) -> Result<(), PluginError>;
    fn process(&self, input: &PluginInput) -> Result<PluginOutput, PluginError>;
    fn cleanup(&mut self) -> Result<(), PluginError>;
}

// Plugin configuration
struct PluginConfig {
    name: String,
    version: String,
    settings: HashMap<String, Value>,
    hooks: Vec<Hook>,
}

// Plugin input/output
struct PluginInput {
    data_type: String,
    content: Value,
    metadata: HashMap<String, Value>,
}

struct PluginOutput {
    success: bool,
    data: Option<Value>,
    errors: Vec<String>,
    warnings: Vec<String>,
}
```

## 🌐 Integration APIs

### MCP Protocol
```protobuf
// Model Context Protocol service
service MCPService {
    rpc Initialize(InitializeRequest) returns (InitializeResponse);
    rpc ListResources(ListResourcesRequest) returns (ListResourcesResponse);
    rpc ReadResource(ReadResourceRequest) returns (ReadResourceResponse);
    rpc WriteResource(WriteResourceRequest) returns (WriteResourceResponse);
    rpc ListTools(ListToolsRequest) returns (ListToolsResponse);
    rpc CallTool(CallToolRequest) returns (CallToolResponse);
}
```

### Action Protocol
```rust
// Action protocol structures
struct ActionIntent {
    id: String,
    description: String,
    scope: String,
    actions: Vec<Action>,
    status: IntentStatus,
    created_date: DateTime<Utc>,
    approved_date: Option<DateTime<Utc>>,
    executed_date: Option<DateTime<Utc>>,
}

struct Action {
    id: String,
    action_type: ActionType,
    target: String,
    parameters: HashMap<String, Value>,
    safety_checks: Vec<SafetyCheck>,
}
```

## 🔒 Security Reference

### Authentication
```yaml
# Authentication configuration
authentication:
  enabled: true
  method: "api_key"  # api_key, oauth, jwt, none
  api_key:
    header: "X-API-Key"
    env_var: "RHEMA_API_KEY"
  oauth:
    provider: "github"
    client_id: "${GITHUB_CLIENT_ID}"
    client_secret: "${GITHUB_CLIENT_SECRET}"
```

### Authorization
```yaml
# Authorization configuration
authorization:
  enabled: true
  model: "rbac"  # rbac, abac, none
  roles:
    - name: "admin"
      permissions: ["*"]
    - name: "developer"
      permissions: ["read", "write", "delete"]
    - name: "viewer"
      permissions: ["read"]
```

### Encryption
```yaml
# Encryption configuration
encryption:
  enabled: false
  algorithm: "aes-256-gcm"
  key_source: "env"  # env, file, vault
  key_env_var: "RHEMA_ENCRYPTION_KEY"
  key_file: "/path/to/key"
```

## 📈 Performance Reference

### Caching
```yaml
# Cache configuration
cache:
  enabled: true
  backend: "memory"  # memory, redis, file
  ttl: 3600
  max_size: "100MB"
  redis:
    url: "redis://localhost:6379"
    database: 0
```

### Query Optimization
```yaml
# Query optimization
query_optimization:
  enabled: true
  index_strategy: "auto"  # auto, manual, none
  parallel_processing: true
  result_limit: 1000
  timeout: 30
```

## 🔍 Monitoring Reference

### Metrics
```yaml
# Metrics configuration
metrics:
  enabled: true
  backend: "prometheus"  # prometheus, statsd, none
  interval: 60
  prometheus:
    port: 9090
    path: "/metrics"
```

### Health Checks
```yaml
# Health check configuration
health_checks:
  enabled: true
  interval: 300
  timeout: 30
  checks:
    - name: "database"
      type: "http"
      url: "http://localhost:5432/health"
    - name: "cache"
      type: "redis"
      url: "redis://localhost:6379"
```

## 📚 Related Documentation

- **[Getting Started](../getting-started/)** - Installation and initial setup
- **[User Guide](../user-guide/)** - User documentation and guides
- **[Core Features](../core-features/)** - Feature documentation
- **[Examples](../examples/)** - Usage examples and patterns
- **[Architecture](../architecture/)** - System architecture and design

## 🤝 Contributing to API Reference

When contributing to the API reference:

1. **Keep it accurate**: Ensure all specifications are current and correct
2. **Provide examples**: Include practical examples for all APIs
3. **Be comprehensive**: Cover all parameters, return values, and error cases
4. **Maintain consistency**: Follow established patterns and conventions
5. **Update schemas**: Keep schemas synchronized with implementation

For questions about the API reference or to report inaccuracies, please open an issue in the repository. 