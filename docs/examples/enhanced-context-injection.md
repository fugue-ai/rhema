# Enhanced Context Injection System

The enhanced context injection system automatically detects task types and selects appropriate context for prompt patterns.

## Task Types

The system supports the following task types:

- **CodeReview** - Code review and quality assurance
- **BugFix** - Bug fixing and debugging
- **FeatureDevelopment** - New feature development
- **Testing** - Test writing and test-related tasks
- **Documentation** - Documentation writing and updates
- **Refactoring** - Code refactoring and restructuring
- **SecurityReview** - Security-focused reviews and audits
- **PerformanceOptimization** - Performance improvements
- **DependencyUpdate** - Dependency and package updates
- **Deployment** - Deployment and infrastructure tasks
- **Custom** - Custom task types

## Automatic Task Detection

The system automatically detects task types based on:

1. **Git Status** - Analyzes modified files and commit messages
2. **File Types** - Examines file extensions and paths
3. **Directory Structure** - Looks at folder names and organization

### Detection Examples

```bash
# Git status detection
modified: src/auth/security.rs  # → SecurityReview
modified: tests/user_test.rs    # → Testing
modified: docs/README.md        # → Documentation

# File type detection
*.test.js, *.spec.js           # → Testing
*.md, *.txt                    # → Documentation
Cargo.lock, package-lock.json  # → DependencyUpdate
```

## Context Injection Rules

Each task type has predefined injection rules that specify:

- **Context Files** - Which `.rhema/` files to load
- **Injection Method** - How to combine context with prompts
- **Priority** - Rule precedence when multiple rules match
- **Additional Context** - Task-specific guidance

### Default Rules

```yaml
# Code Review
task_type: CodeReview
context_files: ["patterns.yaml", "knowledge.yaml"]
injection_method: TemplateVariable
priority: 1
additional_context: "Focus on code quality, best practices, and potential issues."

# Bug Fix
task_type: BugFix
context_files: ["knowledge.yaml", "decisions.yaml"]
injection_method: Prepend
priority: 2
additional_context: "Consider previous bug fixes and known issues."

# Security Review
task_type: SecurityReview
context_files: ["patterns.yaml", "knowledge.yaml"]
injection_method: Prepend
priority: 3
additional_context: "Focus on security vulnerabilities, authentication, and data protection."
```

## Usage Examples

### Basic Testing

```bash
# Test a prompt pattern with automatic task detection
rhema prompt test "Code Review"

# Test with specific task type
rhema prompt test "Code Review" --task-type security

# Test with explicit task type
rhema prompt test-with-task "Code Review" security
```

### Context Rules Management

```bash
# List all context injection rules
rhema context-rules list

# Test context injection for a specific task type
rhema context-rules test security

# Add a custom context injection rule (not yet persisted)
rhema context-rules add "custom_task" \
  --context-files "patterns.yaml,knowledge.yaml" \
  --injection-method prepend \
  --priority 5 \
  --additional-context "Custom task-specific guidance"
```

### Interactive Testing

```bash
# Start interactive mode
rhema interactive --advanced

# Test context injection
builder prompt
# Then test with: prompt test "Code Review" --task-type security
```

## Context File Selection

The system intelligently selects context files based on task type:

| Task Type | Context Files | Rationale |
|-----------|---------------|-----------|
| CodeReview | patterns.yaml, knowledge.yaml | Best practices and domain knowledge |
| BugFix | knowledge.yaml, decisions.yaml | Previous fixes and architectural decisions |
| Testing | patterns.yaml | Testing patterns and conventions |
| Documentation | knowledge.yaml | Domain knowledge and terminology |
| SecurityReview | patterns.yaml, knowledge.yaml | Security patterns and vulnerabilities |
| PerformanceOptimization | patterns.yaml | Performance patterns and benchmarks |
| Refactoring | patterns.yaml, decisions.yaml | Architectural patterns and decisions |
| FeatureDevelopment | patterns.yaml, knowledge.yaml, decisions.yaml | Full context for new features |

## Injection Methods

Three injection methods are supported:

1. **Prepend** - Add context before the prompt template
2. **Append** - Add context after the prompt template  
3. **TemplateVariable** - Replace `{{CONTEXT}}` placeholder in template

### Example Output

```bash
$ rhema prompt test "Code Review" --task-type security

🧪 Testing prompt pattern 'Code Review':
============================================================
=== patterns.yaml ===
# Security patterns
security_patterns:
  input_validation:
    description: "Always validate user input"
    implementation: ["Use regex validation", "Sanitize input"]

=== knowledge.yaml ===
# Security knowledge
security_vulnerabilities:
  - "SQL injection in user input"
  - "XSS in form submissions"

=== Task Context ===
Focus on security vulnerabilities, authentication, and data protection.

=== Task Type ===
Current task: SecurityReview
Context files: patterns.yaml, knowledge.yaml

Please review this code for security issues:
{{CONTEXT}}

Let me know if there are any security concerns.
============================================================
Injection method: TemplateVariable
Success rate: 0.85
Task type: SecurityReview
```

## Advanced Features

### Custom Task Types

You can create custom task types for specialized workflows:

```bash
# Test with custom task type
rhema prompt test "Code Review" --task-type "ml_model_review"
```

### Priority System

Rules with higher priority take precedence when multiple rules match:

```yaml
# High priority rule for security in authentication code
task_type: SecurityReview
priority: 5
additional_context: "Extra security focus for authentication code"

# Lower priority general security rule
task_type: SecurityReview  
priority: 3
additional_context: "General security review"
```

## Future Enhancements

Planned improvements include:

- **Persistent Custom Rules** - Save custom injection rules to `.rhema/context-rules.yaml`
- **Git Integration** - Real git status analysis using git2
- **AI-Powered Detection** - Use AI to suggest task types based on code changes
- **Rule Templates** - Pre-built rule templates for common workflows
- **Rule Validation** - Validate that specified context files exist
- **Performance Optimization** - Cache context loading for better performance

## Troubleshooting

### Common Issues

1. **No context files found** - Ensure `.rhema/` directory exists with context files
2. **Task type not detected** - Check git status and file types in current directory
3. **Rule not matching** - Verify task type spelling and check rule priorities

### Debug Mode

Enable debug output to see task detection details:

```bash
RUST_LOG=debug rhema prompt test "Code Review"
```

This will show:
- Git status analysis
- File type detection
- Rule matching process
- Context loading details 