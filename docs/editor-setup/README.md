# Editor Setup Guides

This directory contains guides for setting up various editors and IDEs to work effectively with GACP (Git-Based Agent Context Protocol) projects.

## Available Guides

### [Cursor Setup Guide](cursor.md)
Complete guide for configuring Cursor to work with GACP, including:
- Installation and configuration
- AI-powered workflows
- Custom snippets and keyboard shortcuts
- Best practices for context-aware development

### [VS Code Setup Guide](vscode.md)
Comprehensive VS Code configuration for GACP development:
- Extension recommendations and configuration
- Workspace settings and tasks
- Git integration and debugging setup
- AI assistance with GitHub Copilot

### [IntelliJ IDEA Setup Guide](intellij.md)
Complete IntelliJ IDEA setup for GACP development:
- Plugin installation and configuration
- External tools and run configurations
- Live templates for GACP files
- Git integration and workflow automation

### [Vim/Neovim Setup Guide](vim.md)
Terminal-based editor setup for GACP development:
- Plugin management and configuration
- Custom commands and functions
- Git integration and hooks
- Language server support

### [Sublime Text Setup Guide](sublime.md)
Lightweight editor configuration for GACP development:
- Package installation and settings
- Custom snippets and build systems
- Git integration and key bindings
- Workflow automation

## Coming Soon

We plan to add setup guides for other editors:

- **Emacs** - Extensible editor setup
- **Atom** - GitHub's editor (if still maintained)
- **Nano** - Simple terminal editor

## Contributing

If you'd like to contribute a setup guide for your preferred editor:

1. Create a new markdown file named `[editor-name].md`
2. Follow the structure of the existing guides
3. Include installation, configuration, and workflow sections
4. Add the guide to this README index
5. Submit a pull request

## General Editor Setup Tips

Regardless of your editor choice, these general principles apply:

### 1. YAML Support
Ensure your editor has good YAML support for editing GACP protocol files:
- Syntax highlighting
- Schema validation
- Auto-completion
- Format on save

### 2. Git Integration
Strong Git integration helps with GACP workflows:
- Visual diff tools
- Branch management
- Commit history
- Merge conflict resolution

### 3. Terminal Integration
Easy access to terminal for running GACP CLI commands:
- Integrated terminal
- Task runners
- Custom commands
- Keyboard shortcuts

### 4. AI Assistance
If your editor supports AI assistance:
- Configure AI to understand GACP context
- Set up rules for context-aware suggestions
- Enable AI to help maintain GACP files

## Common Configuration Files

Most editors can benefit from these configuration files:

### `.cursorrules` (Cursor)
```
# GACP Context Integration
This project uses GACP for structured context management.
Reference .gacp/ files for existing knowledge and decisions.
```

### `.vscode/settings.json` (VS Code)
```json
{
  "yaml.schemas": {
    "schemas/gacp.json": ["**/gacp.yaml", "**/knowledge.yaml", "**/todos.yaml"]
  }
}
```

### `.editorconfig` (Universal)
```ini
[*.{yaml,yml}]
indent_style = space
indent_size = 2
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true
```

## Getting Help

- [GACP Protocol Documentation](../schemas/)
- [CLI Command Reference](../README.md#cli-command-reference)
- [GitHub Issues](https://github.com/fugue-ai/gacp/issues)
- [GitHub Discussions](https://github.com/fugue-ai/gacp/discussions) 