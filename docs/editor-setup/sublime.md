# Setting Up Sublime Text for GACP Development

This guide will help you configure Sublime Text to work effectively with GACP (Git-Based Agent Context Protocol) projects. Sublime Text's speed, extensibility, and powerful text editing capabilities make it an excellent choice for GACP development.

## Prerequisites

- [Sublime Text](https://www.sublimetext.com/) installed on your system
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

## Sublime Text Configuration

### 1. Install Package Control

Package Control is essential for managing Sublime Text packages:

1. Open Sublime Text
2. Go to **Tools** → **Install Package Control**
3. Restart Sublime Text

### 2. Install Recommended Packages

Install these packages via Package Control (**Preferences** → **Package Control** → **Install Package**):

#### Essential Packages
- **[Package Control](https://packagecontrol.io/packages/Package%20Control)** - Package manager (usually pre-installed)
- **[Git](https://packagecontrol.io/packages/Git)** - Git integration
- **[GitGutter](https://packagecontrol.io/packages/GitGutter)** - Git diff indicators
- **[YAML](https://packagecontrol.io/packages/YAML)** - YAML syntax highlighting and validation
- **[Rust Enhanced](https://packagecontrol.io/packages/Rust%20Enhanced)** - Rust language support

#### Recommended Packages
- **[SideBarEnhancements](https://packagecontrol.io/packages/SideBarEnhancements)** - Enhanced sidebar functionality
- **[BracketHighlighter](https://packagecontrol.io/packages/BracketHighlighter)** - Bracket and tag highlighting
- **[AutoFileName](https://packagecontrol.io/packages/AutoFileName)** - Auto-complete filenames
- **[Color Highlighter](https://packagecontrol.io/packages/Color%20Highlighter)** - Color value highlighting
- **[TrailingSpaces](https://packagecontrol.io/packages/TrailingSpaces)** - Highlight trailing spaces
- **[Alignment](https://packagecontrol.io/packages/Alignment)** - Align text and code

### 3. Configure Sublime Text Settings

#### User Settings

Open **Preferences** → **Settings** and add these settings:

```json
{
  "auto_complete": true,
  "auto_complete_commit_on_tab": true,
  "auto_complete_delay": 50,
  "auto_complete_selector": "source - comment",
  "auto_complete_triggers": [
    {
      "selector": "source.yaml",
      "characters": ":"
    }
  ],
  "color_scheme": "Packages/User/SublimeLinter/Monokai (SL).tmTheme",
  "draw_white_space": "selection",
  "ensure_newline_at_eof_on_save": true,
  "font_face": "JetBrains Mono",
  "font_size": 12,
  "highlight_line": true,
  "highlight_modified_tabs": true,
  "ignored_packages": [
    "Vintage"
  ],
  "indent_guide_options": [
    "draw_normal",
    "draw_active"
  ],
  "line_numbers": true,
  "margin": 0,
  "match_brackets": true,
  "match_selection": true,
  "match_tags": true,
  "rulers": [80, 100],
  "save_on_focus_lost": true,
  "scroll_past_end": false,
  "show_encoding": true,
  "show_line_endings": true,
  "spell_check": false,
  "tab_size": 2,
  "translate_tabs_to_spaces": true,
  "trim_automatic_white_space": true,
  "trim_trailing_white_space_on_save": true,
  "word_wrap": false
}
```

#### YAML-Specific Settings

Create a YAML syntax-specific settings file:

1. Open a YAML file
2. Go to **View** → **Syntax** → **YAML**
3. Go to **Preferences** → **Settings - More** → **Syntax Specific - User**

Add these settings:

```json
{
  "tab_size": 2,
  "translate_tabs_to_spaces": true,
  "rulers": [80, 100],
  "word_wrap": false,
  "auto_complete_triggers": [
    {
      "selector": "source.yaml",
      "characters": ":"
    }
  ]
}
```

### 4. Configure Package Settings

#### GitGutter Settings

Open **Preferences** → **Package Settings** → **GitGutter** → **Settings - User**:

```json
{
  "show_line_annotation": true,
  "show_untracked": true,
  "show_deleted": true,
  "show_modified": true,
  "show_inserted": true,
  "show_ignored": false,
  "show_untracked_file_icon": true,
  "show_deleted_file_icon": true,
  "show_modified_file_icon": true,
  "show_inserted_file_icon": true,
  "show_ignored_file_icon": false,
  "show_status_bar_text": true,
  "show_status_bar_icon": true,
  "show_minimap": true,
  "show_in_minimap": true,
  "show_in_side_bar": true,
  "show_in_status_bar": true,
  "show_in_gutter": true,
  "show_in_overview": true,
  "show_in_quick_panel": true,
  "show_in_command_palette": true,
  "show_in_menu": true,
  "show_in_context_menu": true,
  "show_in_sidebar_context_menu": true,
  "show_in_tab_context_menu": true,
  "show_in_gutter_context_menu": true,
  "show_in_overview_context_menu": true,
  "show_in_quick_panel_context_menu": true,
  "show_in_command_palette_context_menu": true,
  "show_in_menu_context_menu": true
}
```

#### Rust Enhanced Settings

Open **Preferences** → **Package Settings** → **Rust Enhanced** → **Settings - User**:

```json
{
  "rust_syntax_checking": true,
  "rust_syntax_checking_method": "check",
  "rust_syntax_checking_extra_env": {
    "RUST_BACKTRACE": "1"
  },
  "rust_format_on_save": true,
  "rust_clippy": true,
  "rust_clippy_extra_env": {
    "RUST_BACKTRACE": "1"
  }
}
```

## Workflow Integration

### 1. Initialize a GACP Scope

1. Open your project in Sublime Text
2. Open the Command Palette (**Ctrl+Shift+P** or **Cmd+Shift+P**)
3. Type "Terminal" and select **Terminal: Open Default**
4. Run `gacp init`

This creates the initial `.gacp/` directory with template files.

### 2. Configure AI Context

Create a `.copilot` file in your project root:

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

#### GACP Todo Snippet

1. Go to **Tools** → **Developer** → **New Snippet**
2. Replace the content with:

```xml
<snippet>
    <content><![CDATA[
- id: "todo-${1:001}"
  title: "${2:Todo title}"
  description: "${3:Detailed description}"
  status: ${4|pending,in_progress,completed,blocked|}
  priority: ${5|low,medium,high,critical|}
  assigned_to: "${6:assignee}"
  created_at: "${7:$CURRENT_YEAR}-${8:$CURRENT_MONTH}-${9:$CURRENT_DATE}T${10:$CURRENT_HOUR}:${11:$CURRENT_MINUTE}:00Z"
  tags: [${12:tag1, tag2}]
  related_components: [${13:component1, component2}]
]]></content>
    <tabTrigger>gacp-todo</tabTrigger>
    <scope>source.yaml</scope>
    <description>Create a new GACP todo item</description>
</snippet>
```

#### GACP Insight Snippet

Create another snippet:

```xml
<snippet>
    <content><![CDATA[
- finding: "${1:Insight finding}"
  impact: "${2:Impact description}"
  solution: "${3:Proposed solution}"
  confidence: ${4|low,medium,high|}
  evidence: [${5:evidence1, evidence2}]
  related_files: [${6:file1, file2}]
  category: ${7|performance,security,architecture,user_experience|}
  recorded_at: "${8:$CURRENT_YEAR}-${9:$CURRENT_MONTH}-${10:$CURRENT_DATE}T${11:$CURRENT_HOUR}:${12:$CURRENT_MINUTE}:00Z"
]]></content>
    <tabTrigger>gacp-insight</tabTrigger>
    <scope>source.yaml</scope>
    <description>Record a new GACP insight</description>
</snippet>
```

#### GACP Decision Snippet

Create another snippet:

```xml
<snippet>
    <content><![CDATA[
- id: "decision-${1:001}"
  title: "${2:Decision title}"
  description: "${3:Detailed description}"
  status: ${4|proposed,approved,rejected,deprecated|}
  rationale: "${5:Decision rationale}"
  alternatives_considered: [${6:alt1, alt2}]
  impact: "${7:Impact description}"
  decided_at: "${8:$CURRENT_YEAR}-${9:$CURRENT_MONTH}-${10:$CURRENT_DATE}T${11:$CURRENT_HOUR}:${12:$CURRENT_MINUTE}:00Z"
]]></content>
    <tabTrigger>gacp-decision</tabTrigger>
    <scope>source.yaml</scope>
    <description>Record a new GACP architectural decision</description>
</snippet>
```

### 4. Save the Snippets

Save these snippets in your User snippets directory:
- **Windows**: `%APPDATA%\Sublime Text 3\Packages\User\`
- **macOS**: `~/Library/Application Support/Sublime Text 3/Packages/User/`
- **Linux**: `~/.config/sublime-text-3/Packages/User/`

## Git Integration

### 1. Git Package Configuration

The Git package provides basic Git functionality:

- **Git: Status** - Show Git status
- **Git: Add** - Stage files
- **Git: Commit** - Commit changes
- **Git: Push** - Push to remote
- **Git: Pull** - Pull from remote
- **Git: Log** - Show commit history

### 2. GitGutter Features

GitGutter provides visual indicators:

- **Modified lines** - Yellow dots
- **Added lines** - Green dots
- **Deleted lines** - Red dots
- **Ignored lines** - Gray dots

### 3. Git Hooks Setup

Configure Git hooks for GACP validation:

#### Pre-commit Hook
Create a `.git/hooks/pre-commit` file:

```bash
#!/bin/sh
# GACP Pre-commit Hook

echo "Running GACP validation..."

# Run GACP validation
if command -v gacp >/dev/null 2>&1; then
    if ! gacp validate --recursive; then
        echo "GACP validation failed. Please fix issues before committing."
        exit 1
    fi
    echo "GACP validation passed."
else
    echo "GACP CLI not found. Skipping validation."
fi
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Custom Build Systems

### 1. GACP Build System

Create a custom build system for GACP commands:

1. Go to **Tools** → **Build System** → **New Build System**
2. Add this configuration:

```json
{
  "cmd": ["gacp", "$file_name"],
  "selector": "source.yaml",
  "working_dir": "$file_path",
  "variants": [
    {
      "name": "Validate",
      "cmd": ["gacp", "validate", "--recursive"],
      "working_dir": "$project_path"
    },
    {
      "name": "Health",
      "cmd": ["gacp", "health"],
      "working_dir": "$project_path"
    },
    {
      "name": "Scopes",
      "cmd": ["gacp", "scopes"],
      "working_dir": "$project_path"
    },
    {
      "name": "Query",
      "cmd": ["gacp", "query", "$file_name"],
      "working_dir": "$project_path"
    }
  ]
}
```

3. Save as `GACP.sublime-build`

### 2. Rust Build System

For Rust development, create a Rust build system:

```json
{
  "cmd": ["cargo", "build"],
  "working_dir": "$project_path",
  "variants": [
    {
      "name": "Run",
      "cmd": ["cargo", "run"]
    },
    {
      "name": "Test",
      "cmd": ["cargo", "test"]
    },
    {
      "name": "Check",
      "cmd": ["cargo", "check"]
    },
    {
      "name": "Clippy",
      "cmd": ["cargo", "clippy"]
    }
  ]
}
```

## Keyboard Shortcuts

### 1. Custom Key Bindings

Create custom key bindings:

1. Go to **Preferences** → **Key Bindings**
2. Add these bindings:

```json
[
  {
    "keys": ["ctrl+shift+g", "v"],
    "command": "exec",
    "args": {
      "cmd": ["gacp", "validate", "--recursive"],
      "working_dir": "$project_path"
    }
  },
  {
    "keys": ["ctrl+shift+g", "h"],
    "command": "exec",
    "args": {
      "cmd": ["gacp", "health"],
      "working_dir": "$project_path"
    }
  },
  {
    "keys": ["ctrl+shift+g", "s"],
    "command": "exec",
    "args": {
      "cmd": ["gacp", "scopes"],
      "working_dir": "$project_path"
    }
  },
  {
    "keys": ["ctrl+shift+g", "i"],
    "command": "exec",
    "args": {
      "cmd": ["gacp", "init"],
      "working_dir": "$project_path"
    }
  }
]
```

### 2. Useful Default Shortcuts

- **Ctrl+Shift+P** - Command Palette
- **Ctrl+P** - Quick Open
- **Ctrl+Shift+F** - Find in Files
- **Ctrl+R** - Goto Symbol
- **Ctrl+G** - Goto Line
- **Ctrl+D** - Select Next Occurrence
- **Ctrl+L** - Select Line
- **Ctrl+Shift+K** - Delete Line
- **Ctrl+Shift+D** - Duplicate Line
- **Ctrl+J** - Join Lines
- **Ctrl+Shift+J** - Split Selection into Lines

## Best Practices

### 1. Regular Context Maintenance

- Run `gacp validate --recursive` before commits
- Update knowledge files when discovering new insights
- Record decisions as they're made, not after the fact
- Keep todos current and accurate

### 2. Sublime Text Specific

- Use multiple cursors for repetitive edits
- Leverage Sublime Text's powerful search and replace
- Use snippets for consistent GACP file creation
- Take advantage of the command palette for quick access

### 3. Team Coordination

- Commit GACP files with related code changes
- Use GACP context in code reviews
- Share insights and decisions through GACP files
- Use cross-scope queries for project-wide coordination

## Troubleshooting

### Common Issues

1. **YAML validation errors**: Ensure your GACP files follow the schema in `schemas/gacp.json`
2. **Missing context**: Run `gacp health` to check scope completeness
3. **Package not working**: Check package installation and configuration
4. **Build system errors**: Ensure GACP CLI is in your PATH
5. **Git integration issues**: Check Git package settings

### Getting Help

- Run `gacp --help` for command documentation
- Check the [GACP README](../README.md) for protocol details
- Use `gacp validate --recursive` to identify issues
- Review the [protocol schemas](../schemas/) for file formats
- Check Sublime Text documentation and package help

## Next Steps

1. **Initialize your first scope**: `gacp init`
2. **Explore existing context**: `gacp scopes` and `gacp query`
3. **Start recording knowledge**: Use `gacp insight record`
4. **Set up team workflows**: Share GACP practices with your team
5. **Integrate with CI/CD**: Add GACP validation to your build pipeline

For more advanced usage, see the [GACP CLI Reference](../README.md#cli-command-reference), [Protocol Documentation](../schemas/), and [Rust Development Setup](../development/rust-setup.md). 