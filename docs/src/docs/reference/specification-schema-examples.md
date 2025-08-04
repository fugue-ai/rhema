# Specification Schema Examples


## rhema.yaml (Scope Definition)


```yaml
rhema:
  version: "1.0.0"
  scope:
    type: "service"
    name: "user-service"
    description: "User management and authentication service"
    boundaries:
      includes: ["src/**", "tests/**"]
      excludes: ["docs/**", "*.md"]
    dependencies:
      parent: "../shared"
      children: ["../user-api", "../user-ui"]
    responsibilities:

      - "User authentication"

      - "User profile management"

      - "Password reset functionality"
    tech:
      primary_languages: ["Rust", "TypeScript"]
      frameworks: ["Actix", "React"]
      databases: ["PostgreSQL"]
  tooling:
    focus_areas:

      - "Security best practices"

      - "Performance optimization"
    recommended_analysis:

      - "Dependency analysis"

      - "Security audit"
```

## knowledge.yaml (Knowledge Base)


```yaml
components:
  user_service:
    description: "Core user management service"
    key_files: ["src/main.rs", "src/models.rs"]
    interfaces:
      auth:
        endpoints:

          - path: "/auth/login"
            method: "POST"
            purpose: "User authentication"
        consumers: ["web-app", "mobile-app"]
    dependencies:
      internal: ["shared", "database"]
      external: ["redis", "jwt"]
    known_issues:

      - issue: "Memory leak in session handling"
        severity: "medium"
        impact: "Gradual performance degradation"
        workaround: "Restart service weekly"
        discovered: "2024-01-15"
    complexity: "medium"
    last_analyzed: "2024-01-20"

architecture:
  patterns:
    repository:
      usage: "required"
      effectiveness: "high"
      description: "Data access abstraction layer"
      examples: ["UserRepository", "SessionRepository"]
      conventions: "Use async/await for all database operations"

insights:
  performance:

    - finding: "Database queries are not optimized"
      impact: "High latency on user operations"
      solution: "Add database indexes and query optimization"
      confidence: "high"
      evidence: ["Query logs", "Performance metrics"]
      related_files: ["src/repository.rs", "migrations/"]
```

## todos.yaml (Work Items)


```yaml
todos:

  - id: "todo-001"
    title: "Implement rate limiting"
    description: "Add rate limiting to prevent abuse"
    status: in_progress
    priority: high
    assigned_to: "alice"
    due_date: "2024-02-01T00:00:00Z"
    created_at: "2024-01-15T09:00:00Z"
    tags: ["security", "performance"]
    related_components: ["auth-service", "api-gateway"]
``` 