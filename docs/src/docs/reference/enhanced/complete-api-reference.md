# Complete API Reference

This document provides comprehensive documentation for all public APIs in Rhema, including the CLI, library APIs, and integration interfaces.

## 📚 Table of Contents

1. [CLI API Reference](#cli-api-reference)
2. [Library API Reference](#library-api-reference)
3. [Plugin API Reference](#plugin-api-reference)
4. [MCP Protocol Reference](#mcp-protocol-reference)
5. [Action Protocol Reference](#action-protocol-reference)
6. [gRPC API Reference](#grpc-api-reference)
7. [REST API Reference](#rest-api-reference)
8. [Error Codes Reference](#error-codes-reference)
9. [Type Definitions](#type-definitions)
10. [Version Compatibility](#version-compatibility)

## 🖥️ CLI API Reference

### Core Commands

#### `rhema init`
Initialize a new Rhema project in the current directory.

**Syntax**:
```bash
rhema init [OPTIONS]
```

**Options**:
- `--force`: Overwrite existing configuration
- `--template <TEMPLATE>`: Use a specific template
- `--config <CONFIG>`: Path to configuration file

**Examples**:
```bash
# Initialize a new project
rhema init

# Initialize with force overwrite
rhema init --force

# Initialize with custom template
rhema init --template rust-web
```

**Exit Codes**:
- `0`: Success
- `1`: General error
- `2`: Configuration error
- `3`: Permission denied

#### `rhema todo`
Manage todo items in the project.

**Subcommands**:
- `add`: Add a new todo
- `list`: List todos
- `update`: Update a todo
- `complete`: Mark todo as complete
- `delete`: Delete a todo
- `search`: Search todos

**Examples**:
```bash
# Add a new todo
rhema todo add --title "Implement feature" --priority high

# List all todos
rhema todo list

# Update a todo
rhema todo update --id "TODO-001" --status "in-progress"

# Complete a todo
rhema todo complete --id "TODO-001" --outcome "Successfully implemented"
```

#### `rhema insight`
Manage insights in the project.

**Subcommands**:
- `add`: Add a new insight
- `list`: List insights
- `update`: Update an insight
- `delete`: Delete an insight
- `search`: Search insights

**Examples**:
```bash
# Add a new insight
rhema insight add --title "Performance optimization" --confidence 8

# Search insights
rhema insight search --query "performance"

# List insights by category
rhema insight list --category "architecture"
```

#### `rhema pattern`
Manage patterns in the project.

**Subcommands**:
- `add`: Add a new pattern
- `list`: List patterns
- `update`: Update a pattern
- `delete`: Delete a pattern
- `search`: Search patterns

**Examples**:
```bash
# Add a new pattern
rhema pattern add --name "Error handling" --type "code"

# Search patterns
rhema pattern search --query "error handling"
```

#### `rhema decision`
Manage decisions in the project.

**Subcommands**:
- `add`: Add a new decision
- `list`: List decisions
- `update`: Update a decision
- `delete`: Delete a decision
- `search`: Search decisions

**Examples**:
```bash
# Add a new decision
rhema decision add --title "Use async/await" --status "approved"

# List decisions by status
rhema decision list --status "approved"
```

### Configuration Commands

#### `rhema config`
Manage Rhema configuration.

**Subcommands**:
- `get`: Get configuration value
- `set`: Set configuration value
- `list`: List all configuration
- `validate`: Validate configuration
- `reset`: Reset to defaults

**Examples**:
```bash
# Get a configuration value
rhema config get --global editor.default

# Set a configuration value
rhema config set --global editor.default "code"

# Validate configuration
rhema config validate
```

### Validation Commands

#### `rhema validate`
Validate project configuration and data.

**Syntax**:
```bash
rhema validate [OPTIONS]
```

**Options**:
- `--strict`: Enable strict validation
- `--fix`: Automatically fix issues
- `--report`: Generate validation report

**Examples**:
```bash
# Basic validation
rhema validate

# Strict validation
rhema validate --strict

# Validation with auto-fix
rhema validate --fix
```

#### `rhema health`
Check project health status.

**Syntax**:
```bash
rhema health [OPTIONS]
```

**Options**:
- `--detailed`: Show detailed health information
- `--fix`: Attempt to fix health issues

**Examples**:
```bash
# Basic health check
rhema health

# Detailed health check
rhema health --detailed
```

## 📚 Library API Reference

### Core Types

#### `Rhema`
Main entry point for the Rhema library.

```rust
pub struct Rhema {
    config: Config,
    project_path: PathBuf,
}

impl Rhema {
    /// Create a new Rhema instance
    pub fn new(project_path: &Path) -> Result<Self, RhemaError>;
    
    /// Initialize a new Rhema project
    pub async fn init(&self) -> Result<(), RhemaError>;
    
    /// Add a todo item
    pub async fn todo_add(&self, todo: &Todo) -> Result<(), RhemaError>;
    
    /// List all todos
    pub async fn todo_list(&self) -> Result<Vec<Todo>, RhemaError>;
    
    /// Update a todo item
    pub async fn todo_update(&self, todo: &Todo) -> Result<(), RhemaError>;
    
    /// Delete a todo item
    pub async fn todo_delete(&self, id: &str) -> Result<(), RhemaError>;
    
    /// Search todos
    pub async fn todo_search(&self, query: &str) -> Result<Vec<Todo>, RhemaError>;
    
    /// Add an insight
    pub async fn insight_add(&self, insight: &Insight) -> Result<(), RhemaError>;
    
    /// List all insights
    pub async fn insight_list(&self) -> Result<Vec<Insight>, RhemaError>;
    
    /// Search insights
    pub async fn insight_search(&self, query: &str) -> Result<Vec<Insight>, RhemaError>;
    
    /// Add a pattern
    pub async fn pattern_add(&self, pattern: &Pattern) -> Result<(), RhemaError>;
    
    /// List all patterns
    pub async fn pattern_list(&self) -> Result<Vec<Pattern>, RhemaError>;
    
    /// Add a decision
    pub async fn decision_add(&self, decision: &Decision) -> Result<(), RhemaError>;
    
    /// List all decisions
    pub async fn decision_list(&self) -> Result<Vec<Decision>, RhemaError>;
    
    /// Search across all data
    pub async fn search(&self, query: &str) -> Result<Vec<SearchResult>, RhemaError>;
    
    /// Validate project
    pub async fn validate(&self) -> Result<ValidationResult, RhemaError>;
    
    /// Check project health
    pub async fn health(&self) -> Result<HealthResult, RhemaError>;
}
```

**Example Usage**:
```rust
use rhema::Rhema;
use rhema::types::{Todo, Insight};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Rhema instance
    let rhema = Rhema::new(std::path::Path::new("."))?;
    
    // Initialize project
    rhema.init().await?;
    
    // Add a todo
    let todo = Todo {
        id: "TODO-001".to_string(),
        title: "Implement feature".to_string(),
        description: Some("Add new functionality".to_string()),
        status: "pending".to_string(),
        priority: "high".to_string(),
        assignee: Some("developer".to_string()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        due_date: None,
        tags: vec!["feature".to_string()],
        related: vec![],
    };
    
    rhema.todo_add(&todo).await?;
    
    // Add an insight
    let insight = Insight {
        id: "INSIGHT-001".to_string(),
        title: "Performance consideration".to_string(),
        content: "Consider using async/await".to_string(),
        confidence: 8,
        category: "performance".to_string(),
        tags: vec!["async".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        related: vec![],
    };
    
    rhema.insight_add(&insight).await?;
    
    // Search for items
    let results = rhema.search("performance").await?;
    println!("Found {} results", results.len());
    
    Ok(())
}
```

### Configuration API

#### `Config`
Configuration management for Rhema projects.

```rust
pub struct Config {
    pub project: ProjectConfig,
    pub editor: EditorConfig,
    pub validation: ValidationConfig,
    pub performance: PerformanceConfig,
}

impl Config {
    /// Load configuration from file
    pub fn load(project_path: &Path) -> Result<Self, ConfigError>;
    
    /// Save configuration to file
    pub fn save(&self, project_path: &Path) -> Result<(), ConfigError>;
    
    /// Validate configuration
    pub fn validate(&self) -> Result<ValidationResult, ConfigError>;
    
    /// Get configuration value
    pub fn get(&self, key: &str) -> Option<&Value>;
    
    /// Set configuration value
    pub fn set(&mut self, key: &str, value: Value) -> Result<(), ConfigError>;
}
```

### Error Types

#### `RhemaError`
Main error type for Rhema operations.

```rust
#[derive(Debug, thiserror::Error)]
pub enum RhemaError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Todo not found: {0}")]
    TodoNotFound(String),
    
    #[error("Insight not found: {0}")]
    InsightNotFound(String),
    
    #[error("Pattern not found: {0}")]
    PatternNotFound(String),
    
    #[error("Decision not found: {0}")]
    DecisionNotFound(String),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}
```

## 🔌 Plugin API Reference

### Plugin Interface

#### `Plugin`
Trait for implementing Rhema plugins.

```rust
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;
    
    /// Plugin version
    fn version(&self) -> &str;
    
    /// Plugin description
    fn description(&self) -> &str;
    
    /// Initialize plugin
    async fn init(&self, config: &Config) -> Result<(), PluginError>;
    
    /// Execute plugin command
    async fn execute(&self, command: &str, args: &[String]) -> Result<String, PluginError>;
    
    /// Get plugin help
    fn help(&self) -> &str;
    
    /// Cleanup plugin resources
    async fn cleanup(&self) -> Result<(), PluginError>;
}
```

**Example Plugin**:
```rust
use rhema::plugin::{Plugin, PluginError};
use async_trait::async_trait;

pub struct MyPlugin;

#[async_trait]
impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "my-plugin"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn description(&self) -> &str {
        "A custom Rhema plugin"
    }
    
    async fn init(&self, _config: &Config) -> Result<(), PluginError> {
        println!("My plugin initialized");
        Ok(())
    }
    
    async fn execute(&self, command: &str, args: &[String]) -> Result<String, PluginError> {
        match command {
            "hello" => Ok(format!("Hello, {}!", args.get(0).unwrap_or(&"world".to_string()))),
            _ => Err(PluginError::UnknownCommand(command.to_string())),
        }
    }
    
    fn help(&self) -> &str {
        "my-plugin hello [name] - Say hello to someone"
    }
    
    async fn cleanup(&self) -> Result<(), PluginError> {
        println!("My plugin cleaned up");
        Ok(())
    }
}
```

## 🔄 MCP Protocol Reference

### Model Context Protocol Integration

Rhema implements the Model Context Protocol (MCP) for AI integration.

#### Server Implementation

```rust
use rhema_mcp::{MCPServer, MCPRequest, MCPResponse};

pub struct RhemaMCPServer {
    rhema: Rhema,
}

#[async_trait]
impl MCPServer for RhemaMCPServer {
    async fn handle_request(&self, request: MCPRequest) -> Result<MCPResponse, MCPError> {
        match request {
            MCPRequest::ListResources => {
                let resources = self.rhema.list_resources().await?;
                Ok(MCPResponse::Resources(resources))
            }
            
            MCPRequest::GetResource { uri } => {
                let resource = self.rhema.get_resource(&uri).await?;
                Ok(MCPResponse::Resource(resource))
            }
            
            MCPRequest::ListTools => {
                let tools = self.rhema.list_tools().await?;
                Ok(MCPResponse::Tools(tools))
            }
            
            MCPRequest::CallTool { name, arguments } => {
                let result = self.rhema.call_tool(&name, arguments).await?;
                Ok(MCPResponse::ToolResult(result))
            }
        }
    }
}
```

#### Available Tools

| Tool | Description | Parameters |
|------|-------------|------------|
| `add_todo` | Add a new todo item | `title`, `description`, `priority`, `assignee` |
| `list_todos` | List all todos | `status`, `priority`, `assignee` |
| `add_insight` | Add a new insight | `title`, `content`, `confidence`, `category` |
| `search` | Search across all data | `query` |
| `validate` | Validate project | None |

## ⚡ Action Protocol Reference

### Safe Agent-Assisted Modifications

Rhema implements the Action Protocol for safe AI-assisted code modifications.

#### Action Types

```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    /// Create a new file
    CreateFile {
        path: String,
        content: String,
        permissions: Option<u32>,
    },
    
    /// Modify an existing file
    ModifyFile {
        path: String,
        changes: Vec<FileChange>,
    },
    
    /// Delete a file
    DeleteFile {
        path: String,
    },
    
    /// Execute a command
    ExecuteCommand {
        command: String,
        args: Vec<String>,
        working_dir: Option<String>,
    },
    
    /// Git operation
    GitOperation {
        operation: GitOp,
        args: Vec<String>,
    },
}
```

#### File Changes

```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum FileChange {
    /// Insert text at a specific line
    Insert {
        line: usize,
        text: String,
    },
    
    /// Replace text in a range
    Replace {
        start_line: usize,
        end_line: usize,
        text: String,
    },
    
    /// Delete text in a range
    Delete {
        start_line: usize,
        end_line: usize,
    },
}
```

#### Action Execution

```rust
impl Rhema {
    /// Execute an action with safety checks
    pub async fn execute_action(&self, action: Action) -> Result<ActionResult, ActionError> {
        // Validate action
        self.validate_action(&action).await?;
        
        // Check permissions
        self.check_permissions(&action).await?;
        
        // Create backup
        self.create_backup().await?;
        
        // Execute action
        let result = match action {
            Action::CreateFile { path, content, permissions } => {
                self.create_file(&path, &content, permissions).await?
            }
            Action::ModifyFile { path, changes } => {
                self.modify_file(&path, changes).await?
            }
            Action::DeleteFile { path } => {
                self.delete_file(&path).await?
            }
            Action::ExecuteCommand { command, args, working_dir } => {
                self.execute_command(&command, &args, working_dir).await?
            }
            Action::GitOperation { operation, args } => {
                self.execute_git_operation(operation, &args).await?
            }
        };
        
        // Validate result
        self.validate_result(&result).await?;
        
        Ok(result)
    }
}
```

## 🌐 gRPC API Reference

### Service Definitions

#### RhemaService

```protobuf
service RhemaService {
    // Project management
    rpc InitProject(InitProjectRequest) returns (InitProjectResponse);
    rpc GetProjectInfo(GetProjectInfoRequest) returns (GetProjectInfoResponse);
    
    // Todo management
    rpc AddTodo(AddTodoRequest) returns (AddTodoResponse);
    rpc ListTodos(ListTodosRequest) returns (ListTodosResponse);
    rpc UpdateTodo(UpdateTodoRequest) returns (UpdateTodoResponse);
    rpc DeleteTodo(DeleteTodoRequest) returns (DeleteTodoResponse);
    
    // Insight management
    rpc AddInsight(AddInsightRequest) returns (AddInsightResponse);
    rpc ListInsights(ListInsightsRequest) returns (ListInsightsResponse);
    rpc SearchInsights(SearchInsightsRequest) returns (SearchInsightsResponse);
    
    // Pattern management
    rpc AddPattern(AddPatternRequest) returns (AddPatternResponse);
    rpc ListPatterns(ListPatternsRequest) returns (ListPatternsResponse);
    
    // Decision management
    rpc AddDecision(AddDecisionRequest) returns (AddDecisionResponse);
    rpc ListDecisions(ListDecisionsRequest) returns (ListDecisionsResponse);
    
    // Search and validation
    rpc Search(SearchRequest) returns (SearchResponse);
    rpc Validate(ValidateRequest) returns (ValidateResponse);
    rpc Health(HealthRequest) returns (HealthResponse);
}
```

#### Message Types

```protobuf
message Todo {
    string id = 1;
    string title = 2;
    optional string description = 3;
    string status = 4;
    string priority = 5;
    optional string assignee = 6;
    google.protobuf.Timestamp created_at = 7;
    google.protobuf.Timestamp updated_at = 8;
    optional google.protobuf.Timestamp due_date = 9;
    repeated string tags = 10;
    repeated string related = 11;
}

message Insight {
    string id = 1;
    string title = 2;
    string content = 3;
    int32 confidence = 4;
    string category = 5;
    repeated string tags = 6;
    google.protobuf.Timestamp created_at = 7;
    google.protobuf.Timestamp updated_at = 8;
    repeated string related = 9;
}

message AddTodoRequest {
    Todo todo = 1;
}

message AddTodoResponse {
    bool success = 1;
    optional string error = 2;
    string id = 3;
}
```

## 🌍 REST API Reference

### Endpoints

#### Project Management

```
POST /api/v1/projects
GET /api/v1/projects/{id}
PUT /api/v1/projects/{id}
DELETE /api/v1/projects/{id}
```

#### Todo Management

```
POST /api/v1/todos
GET /api/v1/todos
GET /api/v1/todos/{id}
PUT /api/v1/todos/{id}
DELETE /api/v1/todos/{id}
```

#### Insight Management

```
POST /api/v1/insights
GET /api/v1/insights
GET /api/v1/insights/{id}
PUT /api/v1/insights/{id}
DELETE /api/v1/insights/{id}
```

#### Search

```
GET /api/v1/search?q={query}
POST /api/v1/search
```

### Request/Response Examples

#### Add Todo

**Request**:
```http
POST /api/v1/todos
Content-Type: application/json

{
    "title": "Implement feature",
    "description": "Add new functionality",
    "priority": "high",
    "assignee": "developer",
    "tags": ["feature"]
}
```

**Response**:
```http
HTTP/1.1 201 Created
Content-Type: application/json

{
    "id": "TODO-001",
    "title": "Implement feature",
    "description": "Add new functionality",
    "status": "pending",
    "priority": "high",
    "assignee": "developer",
    "created_at": "2025-01-27T10:00:00Z",
    "updated_at": "2025-01-27T10:00:00Z",
    "tags": ["feature"]
}
```

#### Search

**Request**:
```http
GET /api/v1/search?q=performance
```

**Response**:
```http
HTTP/1.1 200 OK
Content-Type: application/json

{
    "results": [
        {
            "type": "insight",
            "id": "INSIGHT-001",
            "title": "Performance optimization",
            "content": "Consider using async/await",
            "score": 0.95
        },
        {
            "type": "todo",
            "id": "TODO-002",
            "title": "Optimize database queries",
            "score": 0.87
        }
    ],
    "total": 2,
    "query": "performance"
}
```

## ❌ Error Codes Reference

### HTTP Status Codes

| Code | Status | Description |
|------|--------|-------------|
| 200 | OK | Request successful |
| 201 | Created | Resource created successfully |
| 400 | Bad Request | Invalid request data |
| 401 | Unauthorized | Authentication required |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource not found |
| 409 | Conflict | Resource conflict |
| 422 | Unprocessable Entity | Validation error |
| 500 | Internal Server Error | Server error |

### Error Response Format

```json
{
    "error": {
        "code": "VALIDATION_ERROR",
        "message": "Invalid todo data",
        "details": {
            "field": "title",
            "issue": "Title is required"
        },
        "timestamp": "2025-01-27T10:00:00Z",
        "request_id": "req-12345"
    }
}
```

### Error Codes

| Code | Description | HTTP Status |
|------|-------------|-------------|
| `VALIDATION_ERROR` | Data validation failed | 422 |
| `NOT_FOUND` | Resource not found | 404 |
| `PERMISSION_DENIED` | Insufficient permissions | 403 |
| `CONFLICT` | Resource conflict | 409 |
| `INTERNAL_ERROR` | Internal server error | 500 |
| `CONFIGURATION_ERROR` | Configuration error | 500 |
| `FILE_SYSTEM_ERROR` | File system error | 500 |
| `SERIALIZATION_ERROR` | Serialization error | 500 |

## 📋 Type Definitions

### Core Types

```rust
/// Todo item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub assignee: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub related: Vec<String>,
}

/// Insight item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub id: String,
    pub title: String,
    pub content: String,
    pub confidence: u8,
    pub category: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub related: Vec<String>,
}

/// Pattern item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: String,
    pub name: String,
    pub description: String,
    pub pattern_type: String,
    pub examples: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub related: Vec<String>,
}

/// Decision item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub rationale: String,
    pub alternatives: Vec<String>,
    pub impact: String,
    pub date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub related: Vec<String>,
}
```

### Configuration Types

```rust
/// Project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub auto_backup: bool,
    pub backup_interval: Option<String>,
}

/// Editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub default: String,
    pub auto_open: bool,
    pub theme: Option<String>,
    pub font_size: Option<u32>,
}

/// Validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub strict: bool,
    pub auto_fix: bool,
    pub rules: Vec<ValidationRule>,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub cache_enabled: bool,
    pub cache_size: String,
    pub parallel_processing: bool,
    pub max_memory: Option<String>,
}
```

## 🔄 Version Compatibility

### API Versioning

Rhema uses semantic versioning for API compatibility:

- **Major version**: Breaking changes
- **Minor version**: New features, backward compatible
- **Patch version**: Bug fixes, backward compatible

### Version Matrix

| Rhema Version | CLI API | Library API | gRPC API | REST API |
|---------------|---------|-------------|----------|----------|
| 1.0.0 | ✅ | ✅ | ✅ | ✅ |
| 1.1.0 | ✅ | ✅ | ✅ | ✅ |
| 1.2.0 | ✅ | ✅ | ✅ | ✅ |
| 2.0.0 | ❌ | ❌ | ❌ | ❌ |

### Migration Guide

#### From 1.x to 2.0

**Breaking Changes**:
- CLI command structure changed
- Library API reorganized
- Configuration format updated
- Database schema changed

**Migration Steps**:
1. Backup existing data
2. Update configuration files
3. Migrate database schema
4. Update client code
5. Test thoroughly

---

**API Version**: 1.0.0  
**Last Updated**: 2025-01-27  
**Next Review**: 2025-02-03 