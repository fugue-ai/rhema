# Setting Up Cursor for GACP Development

This guide will help you configure Cursor to work effectively with GACP (Git-Based Agent Context Protocol) projects. Cursor's AI capabilities make it an excellent choice for GACP development, as it can leverage the structured context files to provide more informed assistance.

## Prerequisites

- [Cursor](https://cursor.sh/) installed on your system
- [GACP CLI](../README.md#installation) installed
- A Git repository (or create one for testing)

## Installation

### 1. Install GACP CLI

First, ensure you have the GACP CLI installed:

```bash
# From Cargo (recommended)
cargo install gacp-cli

# Or build from source
git clone https://github.com/fugue-ai/gacp.git
cd gacp
cargo build --release
```

### 2. Verify Installation

```bash
gacp --version
```

## Cursor Configuration

### 1. Install Recommended Extensions

Cursor works best with GACP when you have these extensions installed:

- **YAML** - For editing GACP protocol files
- **GitLens** - Enhanced Git integration
- **Rust Analyzer** - For Rust development (if contributing to GACP CLI)
- **GitHub Copilot** - AI assistance (optional, Cursor has built-in AI)

### 2. Configure Cursor Settings

Add these settings to your Cursor workspace settings (`.vscode/settings.json`):

```json
{
  "files.associations": {
    "*.gacp.yaml": "yaml",
    "gacp.yaml": "yaml",
    "knowledge.yaml": "yaml",
    "todos.yaml": "yaml",
    "decisions.yaml": "yaml",
    "patterns.yaml": "yaml",
    "conventions.yaml": "yaml"
  },
  "yaml.schemas": {
    "schemas/gacp.json": [
      "**/gacp.yaml",
      "**/knowledge.yaml", 
      "**/todos.yaml",
      "**/decisions.yaml",
      "**/patterns.yaml",
      "**/conventions.yaml"
    ]
  },
  "yaml.validate": true,
  "yaml.format.enable": true,
  "editor.formatOnSave": true,
  "files.exclude": {
    "**/.gacp/temp": true,
    "**/.gacp/cache": true
  },
  "search.exclude": {
    "**/.gacp/temp": true,
    "**/.gacp/cache": true
  }
}
```

### 3. Create Workspace Configuration

Create a `.vscode/tasks.json` file for common GACP operations:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "GACP: Initialize Scope",
      "type": "shell",
      "command": "gacp",
      "args": ["init"],
      "group": "build",
      "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "shared"
      },
      "problemMatcher": []
    },
    {
      "label": "GACP: Validate All",
      "type": "shell", 
      "command": "gacp",
      "args": ["validate", "--recursive"],
      "group": "test",
      "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "shared"
      },
      "problemMatcher": []
    },
    {
      "label": "GACP: Show Health",
      "type": "shell",
      "command": "gacp", 
      "args": ["health"],
      "group": "test",
      "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "shared"
      },
      "problemMatcher": []
    },
    {
      "label": "GACP: List Scopes",
      "type": "shell",
      "command": "gacp",
      "args": ["scopes"],
      "group": "build",
      "presentation": {
        "echo": true,
        "reveal": "always",
        "focus": false,
        "panel": "shared"
      },
      "problemMatcher": []
    }
  ]
}
```

## Workflow Integration

### 1. Initialize a GACP Scope

1. Open your project in Cursor
2. Open the Command Palette (`Cmd/Ctrl + Shift + P`)
3. Run "Tasks: Run Task" and select "GACP: Initialize Scope"
4. Or use the terminal: `gacp init`

This creates the initial `.gacp/` directory with template files.

### 2. Configure AI Context

Cursor's AI can leverage GACP context files for better assistance. Add this to your project's `.cursorrules` file:

```
# GACP Context Integration

This project uses GACP (Git-Based Agent Context Protocol) for structured context management.

## Key Files to Reference:
- .gacp/gacp.yaml - Scope definition and metadata
- .gacp/knowledge.yaml - Domain knowledge and insights  
- .gacp/todos.yaml - Work items and tasks
- .gacp/decisions.yaml - Architecture decisions
- .gacp/patterns.yaml - Design patterns
- .gacp/conventions.yaml - Coding standards

## When Providing Assistance:
1. Check .gacp/knowledge.yaml for existing insights and domain knowledge
2. Review .gacp/decisions.yaml for architectural decisions
3. Consider .gacp/patterns.yaml for established design patterns
4. Follow .gacp/conventions.yaml for coding standards
5. Update relevant GACP files when making significant changes

## Common GACP Commands:
- gacp query "todos WHERE status='in_progress'" - Find active work
- gacp insight record "finding" - Record new insights
- gacp decision record "title" - Record architectural decisions
- gacp validate --recursive - Validate all GACP files
```

### 3. Create Custom Snippets

Add these snippets to `.vscode/snippets/gacp.code-snippets`:

```json
{
  "GACP Todo": {
    "prefix": "gacp-todo",
    "body": [
      "- id: \"todo-${1:001}\"",
      "  title: \"${2:Todo title}\"",
      "  description: \"${3:Detailed description}\"",
      "  status: ${4|pending,in_progress,completed,blocked|}",
      "  priority: ${5|low,medium,high,critical|}",
      "  assigned_to: \"${6:assignee}\"",
      "  created_at: \"${7:$CURRENT_YEAR}-${8:$CURRENT_MONTH}-${9:$CURRENT_DATE}T${10:$CURRENT_HOUR}:${11:$CURRENT_MINUTE}:00Z\"",
      "  tags: [${12:tag1, tag2}]",
      "  related_components: [${13:component1, component2}]"
    ],
    "description": "Create a new GACP todo item"
  },
  "GACP Insight": {
    "prefix": "gacp-insight",
    "body": [
      "- finding: \"${1:Insight finding}\"",
      "  impact: \"${2:Impact description}\"",
      "  solution: \"${3:Proposed solution}\"",
      "  confidence: ${4|low,medium,high|}",
      "  evidence: [${5:evidence1, evidence2}]",
      "  related_files: [${6:file1, file2}]",
      "  category: ${7|performance,security,architecture,user_experience|}",
      "  recorded_at: \"${8:$CURRENT_YEAR}-${9:$CURRENT_MONTH}-${10:$CURRENT_DATE}T${11:$CURRENT_HOUR}:${12:$CURRENT_MINUTE}:00Z\""
    ],
    "description": "Record a new GACP insight"
  },
  "GACP Decision": {
    "prefix": "gacp-decision",
    "body": [
      "- id: \"decision-${1:001}\"",
      "  title: \"${2:Decision title}\"",
      "  description: \"${3:Detailed description}\"",
      "  status: ${4|proposed,approved,rejected,deprecated|}",
      "  rationale: \"${5:Decision rationale}\"",
      "  alternatives_considered: [${6:alt1, alt2}]",
      "  impact: \"${7:Impact description}\"",
      "  decided_at: \"${8:$CURRENT_YEAR}-${9:$CURRENT_MONTH}-${10:$CURRENT_DATE}T${11:$CURRENT_HOUR}:${12:$CURRENT_MINUTE}:00Z\""
    ],
    "description": "Record a new GACP architectural decision"
  }
}
```

## AI-Powered Workflows

### 1. Context-Aware Code Generation

When asking Cursor's AI to generate code:

1. **Reference existing context**: "Based on the patterns in `.gacp/patterns.yaml`, generate..."
2. **Follow established decisions**: "Following the decision in `.gacp/decisions.yaml` about database choice..."
3. **Consider existing insights**: "Given the performance insights in `.gacp/knowledge.yaml`..."

### 2. Automated Context Updates

Use Cursor's AI to help maintain GACP files:

- "Update `.gacp/knowledge.yaml` with insights from this code change"
- "Record this architectural decision in `.gacp/decisions.yaml`"
- "Add a todo item for this technical debt in `.gacp/todos.yaml`"

### 3. Cross-Scope Analysis

For multi-scope projects:

- "Analyze the impact of this change across all GACP scopes"
- "Find todos related to this feature across the entire project"
- "Identify knowledge gaps in the current GACP context"

## Keyboard Shortcuts

Add these to your Cursor keybindings (`.vscode/keybindings.json`):

```json
[
  {
    "key": "ctrl+shift+g i",
    "command": "workbench.action.tasks.runTask",
    "args": "GACP: Initialize Scope"
  },
  {
    "key": "ctrl+shift+g v", 
    "command": "workbench.action.tasks.runTask",
    "args": "GACP: Validate All"
  },
  {
    "key": "ctrl+shift+g h",
    "command": "workbench.action.tasks.runTask", 
    "args": "GACP: Show Health"
  },
  {
    "key": "ctrl+shift+g s",
    "command": "workbench.action.tasks.runTask",
    "args": "GACP: List Scopes"
  }
]
```

## Best Practices

### 1. Regular Context Maintenance

- Run `gacp validate --recursive` before commits
- Update knowledge files when discovering new insights
- Record decisions as they're made, not after the fact
- Keep todos current and accurate

### 2. AI Collaboration

- Use Cursor's AI to help maintain GACP files
- Ask AI to suggest context updates based on code changes
- Leverage AI to find relevant existing context
- Use AI to help identify knowledge gaps

### 3. Team Coordination

- Commit GACP files with related code changes
- Use GACP context in code reviews
- Share insights and decisions through GACP files
- Use cross-scope queries for project-wide coordination

## Troubleshooting

### Common Issues

1. **YAML validation errors**: Ensure your GACP files follow the schema in `schemas/gacp.json`
2. **Missing context**: Run `gacp health` to check scope completeness
3. **AI not using context**: Verify `.cursorrules` is properly configured
4. **Schema not loading**: Check that `schemas/gacp.json` path is correct in settings

### Getting Help

- Run `gacp --help` for command documentation
- Check the [GACP README](../README.md) for protocol details
- Use `gacp validate --recursive` to identify issues
- Review the [protocol schemas](../schemas/) for file formats

## Next Steps

1. **Initialize your first scope**: `gacp init`
2. **Explore existing context**: `gacp scopes` and `gacp query`
3. **Start recording knowledge**: Use `gacp insight record`
4. **Set up team workflows**: Share GACP practices with your team
5. **Integrate with CI/CD**: Add GACP validation to your build pipeline

For more advanced usage, see the [GACP CLI Reference](../README.md#cli-command-reference) and [Protocol Documentation](../schemas/). 