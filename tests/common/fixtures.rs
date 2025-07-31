//! Test fixtures and sample data

use std::collections::HashMap;

/// Test fixtures for Rhema testing
#[allow(dead_code)]
pub struct TestFixtures;

#[allow(dead_code)]
impl TestFixtures {
    /// Get a basic scope definition
    pub fn basic_scope() -> &'static str {
        r#"
name: simple
scope_type: service
description: Basic test scope
version: "1.0.0"
schema_version: "1.0.0"
dependencies: null
"#
    }

    /// Get a complex scope definition
    pub fn complex_scope() -> &'static str {
        r#"
name: complex
scope_type: application
description: Complex test scope with dependencies
version: "2.0.0"
schema_version: "1.0.0"
dependencies:
  - path: "../dependency-1"
    dependency_type: required
    version: "1.0.0"
  - path: "../dependency-2"
    dependency_type: optional
    version: "2.0.0"
"#
    }

    /// Get sample todos data
    pub fn todos_data() -> &'static str {
        r#"
todos:
  - id: "todo-001"
    title: "Implement feature A"
    description: "Add new functionality to the system"
    status: pending
    priority: high
    assigned_to: "developer1"
    due_date: "2024-02-15T10:00:00Z"
    created_at: "2024-01-15T10:00:00Z"
  - id: "todo-002"
    title: "Fix bug in module B"
    description: "Critical bug affecting production"
    status: in_progress
    priority: critical
    assigned_to: "developer2"
    created_at: "2024-01-16T10:00:00Z"
  - id: "todo-003"
    title: "Update documentation"
    description: "Keep docs up to date"
    status: completed
    priority: medium
    assigned_to: "tech-writer"
    created_at: "2024-01-17T10:00:00Z"
    completed_at: "2024-01-20T10:00:00Z"
    outcome: "Documentation updated successfully"
"#
    }

    /// Get sample insights data
    pub fn insights_data() -> &'static str {
        r#"
insights:
  - id: "insight-001"
    title: "Performance bottleneck identified"
    content: "The database query in module X is causing performance issues"
    confidence: 9
    category: "performance"
    tags: ["database", "optimization"]
    created_at: "2024-01-15T10:00:00Z"
  - id: "insight-002"
    title: "Security vulnerability found"
    content: "Input validation is missing in the authentication module"
    confidence: 8
    category: "security"
    tags: ["security", "authentication", "validation"]
    created_at: "2024-01-16T10:00:00Z"
"#
    }

    /// Get sample patterns data
    pub fn patterns_data() -> &'static str {
        r#"
patterns:
  - id: "pattern-001"
    name: "Repository Pattern"
    description: "Use repository pattern for data access"
    pattern_type: "architectural"
    usage: recommended
    effectiveness: 9
    examples:
      - "Data access layer abstraction"
      - "Unit testing with mocks"
    anti_patterns:
      - "Direct database access in controllers"
      - "Mixing business logic with data access"
    created_at: "2024-01-15T10:00:00Z"
  - id: "pattern-002"
    name: "Factory Pattern"
    description: "Use factory pattern for object creation"
    pattern_type: "creational"
    usage: required
    effectiveness: 8
    examples:
      - "Creating different types of connections"
      - "Object instantiation based on configuration"
    created_at: "2024-01-16T10:00:00Z"
"#
    }

    /// Get sample decisions data
    pub fn decisions_data() -> &'static str {
        r#"
decisions:
  - id: "decision-001"
    title: "Choose React for frontend"
    description: "Decision to use React as the primary frontend framework"
    status: approved
    context: "Need to modernize the frontend architecture"
    alternatives:
      - "Vue.js"
      - "Angular"
      - "Vanilla JavaScript"
    rationale: "React has the largest ecosystem and community support"
    consequences:
      - "Need to train team on React"
      - "Larger bundle size"
      - "Better developer experience"
    decided_at: "2024-01-15T10:00:00Z"
    decision_makers: ["tech-lead", "architect"]
  - id: "decision-002"
    title: "Use PostgreSQL for database"
    description: "Decision to use PostgreSQL as the primary database"
    status: implemented
    context: "Need to replace legacy database system"
    alternatives:
      - "MySQL"
      - "MongoDB"
      - "SQLite"
    rationale: "PostgreSQL provides better ACID compliance and advanced features"
    consequences:
      - "Better data integrity"
      - "More complex setup"
      - "Higher resource usage"
    decided_at: "2024-01-16T10:00:00Z"
    decision_makers: ["dba", "architect"]
"#
    }

    /// Get schema definitions
    pub fn schema_definitions() -> &'static str {
        r#"
# Rhema Schema Definitions
schemas:
  scope:
    type: object
    required: [name, scope_type, version]
    properties:
      name:
        type: string
        description: "Scope name and identifier"
      scope_type:
        type: string
        enum: [service, application, library, infrastructure]
        description: "Type of scope"
      version:
        type: string
        pattern: "^\\d+\\.\\d+\\.\\d+$"
        description: "Semantic version"
      description:
        type: string
        description: "Human-readable description"
      dependencies:
        type: array
        items:
          type: object
          required: [path, dependency_type]
          properties:
            path:
              type: string
              description: "Path to dependent scope"
            dependency_type:
              type: string
              enum: [required, optional, peer]
              description: "Type of dependency"
            version:
              type: string
              description: "Version constraint"
  
  todo:
    type: object
    required: [id, title, status, priority, created_at]
    properties:
      id:
        type: string
        pattern: "^[a-zA-Z0-9-_]+$"
        description: "Unique identifier"
      title:
        type: string
        minLength: 1
        maxLength: 200
        description: "Todo title"
      description:
        type: string
        description: "Detailed description"
      status:
        type: string
        enum: [pending, in_progress, blocked, completed, cancelled]
        description: "Current status"
      priority:
        type: string
        enum: [low, medium, high, critical]
        description: "Priority level"
      assigned_to:
        type: string
        description: "Assigned person"
      due_date:
        type: string
        format: date-time
        description: "Due date"
      created_at:
        type: string
        format: date-time
        description: "Creation timestamp"
      completed_at:
        type: string
        format: date-time
        description: "Completion timestamp"
      outcome:
        type: string
        description: "Completion outcome"
"#
    }

    /// Get test queries
    pub fn test_queries() -> HashMap<&'static str, &'static str> {
        let mut queries = HashMap::new();
        queries.insert("simple", "simple");
        queries.insert("filtered", "simple.items WHERE active=true");
        queries.insert("ordered", "simple.items ORDER BY created_at DESC");
        queries.insert("limited", "simple.items LIMIT 5");
        queries.insert(
            "complex",
            "complex.todos WHERE priority=high AND status=pending ORDER BY due_date ASC LIMIT 10",
        );
        queries
    }

    /// Get file structures
    pub fn file_structures() -> HashMap<&'static str, Vec<&'static str>> {
        let mut structures = HashMap::new();
        structures.insert("basic", vec!["rhema.yaml", "simple.yaml"]);
        structures.insert(
            "complex",
            vec![
                "rhema.yaml",
                "todos.yaml",
                "insights.yaml",
                "patterns.yaml",
                "decisions.yaml",
            ],
        );
        structures.insert("nested", vec!["rhema.yaml", "data/", "schemas/", "docs/"]);
        structures
    }

    /// Get git commits
    pub fn git_commits() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Initial commit", "Add basic Rhema structure"),
            ("Add todos", "Add todo management functionality"),
            ("Add insights", "Add insight tracking"),
            ("Add patterns", "Add pattern documentation"),
            ("Add decisions", "Add decision tracking"),
        ]
    }

    /// Get error scenarios
    pub fn error_scenarios() -> HashMap<&'static str, &'static str> {
        let mut scenarios = HashMap::new();
        scenarios.insert("invalid_yaml", "name: scope\n  invalid: yaml: structure");
        scenarios.insert("missing_required", "scope_type: service\nversion: 1.0.0");
        scenarios.insert(
            "invalid_version",
            "name: scope\nscope_type: service\nversion: invalid",
        );
        scenarios.insert("circular_dependency", "name: scope1\ndependencies:\n  - path: ../scope2\nname: scope2\ndependencies:\n  - path: ../scope1");
        scenarios
    }

    /// Get performance data
    pub fn performance_data() -> HashMap<&'static str, usize> {
        let mut data = HashMap::new();
        data.insert("small_dataset", 100);
        data.insert("medium_dataset", 1000);
        data.insert("large_dataset", 10000);
        data.insert("huge_dataset", 100000);
        data
    }
}
