# Prompt Chain Persistence


The prompt chain persistence system allows you to create and manage multi-step workflows that combine multiple prompt patterns in sequence.

## Overview


Prompt chains enable:

- **Multi-step workflows** - Combine multiple prompt patterns in sequence

- **Dependency management** - Define which steps depend on others

- **Variable passing** - Pass data between workflow steps

- **Conditional execution** - Execute steps based on conditions

- **Usage tracking** - Monitor workflow effectiveness and performance

## Workflow Structure


### PromptChain


```yaml

- id: "workflow-1"
  name: "Complete Code Review Workflow"
  description: "Multi-step workflow for comprehensive code reviews"
  steps: [...]                    # Array of ChainStep
  metadata:                       # ChainMetadata
    version: "1.0.0"
    created_at: "2025-01-01T00:00:00Z"
    updated_at: "2025-01-15T10:30:00Z"
    author: "alice@example.com"
    usage_stats: {...}            # ChainUsageStats
    success_criteria: [...]
  tags: ["code-review", "security"]
```

### ChainStep


```yaml

- id: "step-1"
  name: "Initial Code Review"
  description: "Basic code quality review"
  prompt_pattern: "Code Review Request"  # References prompt pattern
  task_type: "code_review"               # Context injection task type
  order: 1                              # Execution order
  required: true                        # Whether step is required
  dependencies: ["step-2"]              # Steps that must complete first
  variables:                            # Step-specific variables
    review_type: "initial"
    focus_areas: "code_quality"
  conditions: null                      # Execution conditions
```

## Usage Examples


### Creating Workflows


```bash
# Create a new workflow


rhema workflow add "Complete Code Review Workflow" \
  --description "Multi-step workflow for comprehensive code reviews" \
  --tags "code-review,security,performance"

# Add steps to the workflow


rhema workflow add-step "Complete Code Review Workflow" \
  "Initial Code Review" \
  "Code Review Request" \
  --task-type "code_review" \
  --description "Basic code quality and best practices review" \
  --required \
  --variables "review_type=initial,focus_areas=code_quality"

rhema workflow add-step "Complete Code Review Workflow" \
  "Security Review" \
  "Code Review Request" \
  --task-type "security_review" \
  --description "Security-focused code review" \
  --required \
  --dependencies "Initial Code Review" \
  --variables "review_type=security,focus_areas=vulnerabilities"
```

### Executing Workflows


```bash
# Execute a workflow


rhema workflow execute "Complete Code Review Workflow"

# Dry run to see what would happen


rhema workflow execute "Complete Code Review Workflow" --dry-run

# Record execution results


rhema workflow record-execution "Complete Code Review Workflow" \
  --successful \
  --execution-time 45.2
```

### Managing Workflows


```bash
# List all workflows


rhema workflow list

# Show detailed workflow information


rhema workflow show "Complete Code Review Workflow"

# List workflows by tags


rhema workflow list --tags "security,code-review"
```

## Workflow Features


### Step Dependencies


Define which steps must complete before others:

```bash
# Step 2 depends on Step 1


rhema workflow add-step "Workflow" "Step 2" "Pattern" \
  --dependencies "Step 1"

# Step 3 depends on both Step 1 and Step 2


rhema workflow add-step "Workflow" "Step 3" "Pattern" \
  --dependencies "Step 1,Step 2"
```

### Step Variables


Pass data between workflow steps:

```bash
# Set variables for a step


rhema workflow add-step "Workflow" "Security Review" "Pattern" \
  --variables "review_type=security,severity=high,focus_areas=vulnerabilities"
```

### Conditional Execution


Make steps optional or conditional:

```bash
# Optional step


rhema workflow add-step "Workflow" "Performance Review" "Pattern" \
  --required false

# Required step


rhema workflow add-step "Workflow" "Security Review" "Pattern" \
  --required true
```

## Example Workflows


### Code Review Workflow


```bash
# Create comprehensive code review workflow


rhema workflow add "Complete Code Review" \
  --description "Multi-step code review with security and performance checks" \
  --tags "code-review,security,performance"

# Step 1: Initial review


rhema workflow add-step "Complete Code Review" \
  "Initial Review" "Code Review Request" \
  --task-type "code_review" \
  --required \
  --variables "review_type=initial"

# Step 2: Security review (depends on Step 1)


rhema workflow add-step "Complete Code Review" \
  "Security Review" "Code Review Request" \
  --task-type "security_review" \
  --required \
  --dependencies "Initial Review" \
  --variables "review_type=security"

# Step 3: Performance review (depends on Step 1, optional)


rhema workflow add-step "Complete Code Review" \
  "Performance Review" "Code Review Request" \
  --task-type "performance_optimization" \
  --required false \
  --dependencies "Initial Review" \
  --variables "review_type=performance"

# Step 4: Documentation update (depends on Steps 1 and 2)


rhema workflow add-step "Complete Code Review" \
  "Documentation Update" "Documentation Update" \
  --task-type "documentation" \
  --required false \
  --dependencies "Initial Review,Security Review" \
  --variables "doc_type=code_documentation"
```

### Bug Fix Workflow


```bash
# Create bug fix workflow


rhema workflow add "Bug Fix and Testing" \
  --description "Workflow for fixing bugs and ensuring proper testing" \
  --tags "bug-fix,testing,quality"

# Step 1: Bug analysis


rhema workflow add-step "Bug Fix and Testing" \
  "Bug Analysis" "Bug Report Template" \
  --task-type "bug_fix" \
  --required \
  --variables "analysis_type=root_cause"

# Step 2: Fix implementation (depends on Step 1)


rhema workflow add-step "Bug Fix and Testing" \
  "Fix Implementation" "Code Review Request" \
  --task-type "bug_fix" \
  --required \
  --dependencies "Bug Analysis" \
  --variables "fix_type=bug_fix"

# Step 3: Test writing (depends on Step 2)


rhema workflow add-step "Bug Fix and Testing" \
  "Test Writing" "Test Writing" \
  --task-type "testing" \
  --required \
  --dependencies "Fix Implementation" \
  --variables "test_type=regression"

# Step 4: Regression testing (depends on Step 3)


rhema workflow add-step "Bug Fix and Testing" \
  "Regression Testing" "Test Execution" \
  --task-type "testing" \
  --required \
  --dependencies "Test Writing" \
  --variables "test_suite=full"
```

## Execution Flow


### Step Execution Order


1. **Dependency Resolution** - Steps are executed in dependency order

2. **Condition Checking** - Optional steps may be skipped based on conditions

3. **Context Injection** - Each step uses its specified task type for context

4. **Variable Substitution** - Step variables are applied to prompt patterns

5. **Result Tracking** - Execution results are tracked for analytics

### Example Execution Output


```bash
$ rhema workflow execute "Complete Code Review"

🔄 Executing workflow 'Complete Code Review':
============================================================
📋 Step 1: Initial Review
   Description: Basic code quality and best practices review
   Prompt pattern: Code Review Request
   Task type: code_review
   Required: true
   Executing...

📋 Step 2: Security Review
   Description: Security-focused code review
   Prompt pattern: Code Review Request
   Task type: security_review
   Required: true
   Executing...

📋 Step 3: Performance Review
   Description: Performance and optimization review
   Prompt pattern: Code Review Request
   Task type: performance_optimization
   Required: false
   Executing...

📋 Step 4: Documentation Update
   Description: Update documentation based on code changes
   Prompt pattern: Documentation Update
   Task type: documentation
   Required: false
   Executing...

✅ Workflow execution completed
   Total steps: 4
   Executed steps: 4
   Execution time: 45.23s

📊 Step Results:
   Initial Review: Success
   Security Review: Success
   Performance Review: Success
   Documentation Update: Success
```

## Usage Statistics


### Tracking Workflow Performance


```bash
# Record successful execution


rhema workflow record-execution "Complete Code Review" \
  --successful \
  --execution-time 45.2

# Record failed execution


rhema workflow record-execution "Complete Code Review" \
  --successful false \
  --execution-time 30.1
```

### Viewing Analytics


```bash
# Show workflow with usage statistics


rhema workflow show "Complete Code Review"
```

Output includes:

- Total executions

- Success rate

- Average execution time

- Last executed timestamp

- Success criteria

## Best Practices


### Workflow Design


1. **Keep steps focused** - Each step should have a single, clear purpose

2. **Use dependencies wisely** - Only add dependencies when truly needed

3. **Make steps optional** - Use `--required false` for non-critical steps

4. **Document success criteria** - Define what makes a workflow successful

### Variable Management


1. **Use descriptive variable names** - `review_type` vs `type`

2. **Pass context between steps** - Use variables to share information

3. **Keep variables simple** - Avoid complex nested structures

### Execution Strategy


1. **Test with dry-run** - Always test workflows before execution

2. **Monitor performance** - Track execution times and success rates

3. **Iterate and improve** - Use analytics to optimize workflows

## Integration with Other Features


### Workflows + Prompt Patterns


Workflows reference existing prompt patterns:

```bash
# Create a prompt pattern first


rhema prompt add "Code Review Request" \
  --template "Please review: {{CONTEXT}}" \
  --injection template_variable

# Then use it in a workflow


rhema workflow add-step "Workflow" "Review" "Code Review Request"
```

### Workflows + Context Injection


Each step can specify a task type for context injection:

```bash
rhema workflow add-step "Workflow" "Security Review" "Pattern" \
  --task-type "security_review"
```

### Workflows + Analytics


Track both individual prompt patterns and overall workflow performance:

```bash
# Record individual prompt usage


rhema prompt record-usage "Code Review Request" true

# Record workflow execution


rhema workflow record-execution "Complete Code Review" true
```

## Troubleshooting


### Common Issues


1. **Missing dependencies** - Ensure all referenced prompt patterns exist

2. **Circular dependencies** - Avoid dependency loops between steps

3. **Invalid task types** - Use valid task types for context injection

### Debugging


```bash
# Check workflow structure


rhema workflow show "Workflow Name"

# Test execution with dry-run


rhema workflow execute "Workflow Name" --dry-run

# Verify prompt patterns exist


rhema prompt list
```

## Future Enhancements


Planned improvements include:

- **Parallel execution** - Execute independent steps in parallel

- **Conditional logic** - Advanced conditions for step execution

- **Error handling** - Graceful handling of step failures

- **Workflow templates** - Pre-built workflow templates

- **Visual workflow editor** - GUI for creating workflows

- **Workflow versioning** - Track workflow changes over time

- **Integration APIs** - Connect workflows to external systems

The prompt chain persistence system provides a powerful way to orchestrate complex AI interactions and maintain consistent, repeatable workflows. 