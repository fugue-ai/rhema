# Setting Up IntelliJ IDEA for Rhema Development


This guide will help you configure IntelliJ IDEA to work effectively with Rhema (Git-Based Agent Context Protocol) projects. IntelliJ's powerful Git integration, YAML support, and extensibility make it an excellent choice for Rhema development, especially for Java/Kotlin projects.

## Prerequisites


- [IntelliJ IDEA](https://www.jetbrains.com/idea/) installed on your system

- [Rhema CLI](../README.md#installation) installed

- A Git repository (or create one for testing)

## Installation


### 1. Install Rhema CLI


First, ensure you have the Rhema CLI installed:

```bash
# From Cargo (recommended)


cargo install rhema-cli

# Or build from source


git clone https://github.com/fugue-ai/rhema.git
cd rhema
cargo build --release
```

### 2. Verify Installation


```bash
rhema --version
```

## IntelliJ Configuration


### 1. Install Recommended Plugins


IntelliJ works best with Rhema when you have these plugins installed:

#### Essential Plugins


- **[YAML/TOML Support](https://plugins.jetbrains.com/plugin/8195-yaml-toml-support)** - YAML language support with schema validation

- **[Git Integration](https://plugins.jetbrains.com/plugin/8183-git-integration)** - Enhanced Git capabilities (usually pre-installed)

- **[Rust](https://plugins.jetbrains.com/plugin/8182-rust)** - Rust language support (for CLI development)

- **[GitHub Copilot](https://plugins.jetbrains.com/plugin/17718-github-copilot)** - AI code assistance (optional)

#### Recommended Plugins


- **[Rainbow Brackets](https://plugins.jetbrains.com/plugin/10080-rainbow-brackets)** - Visual bracket matching

- **[String Manipulation](https://plugins.jetbrains.com/plugin/2162-string-manipulation)** - String and text utilities

- **[Key Promoter X](https://plugins.jetbrains.com/plugin/9792-key-promoter-x)** - Learn keyboard shortcuts

- **[GitToolBox](https://plugins.jetbrains.com/plugin/7499-gittoolbox)** - Enhanced Git features

### 2. Configure IntelliJ Settings


#### File Associations


1. Go to **File** → **Settings** → **Editor** → **File Types**

2. Find **YAML** in the list

3. Add these patterns to **Registered Patterns**:

   - `*.rhema.yaml`

   - `rhema.yaml`

   - `knowledge.yaml`

   - `todos.yaml`

   - `decisions.yaml`

   - `patterns.yaml`

   - `conventions.yaml`

#### YAML Schema Configuration


1. Go to **File** → **Settings** → **Languages & Frameworks** → **Schema and DTDs** → **JSON Schema Mappings**

2. Add a new mapping:

   - **Schema file or URL**: `schemas/rhema.json`

   - **Schema version**: `1.0`

   - **File path pattern**: `**/rhema.yaml`

   - **File path pattern**: `**/knowledge.yaml`

   - **File path pattern**: `**/todos.yaml`

   - **File path pattern**: `**/decisions.yaml`

   - **File path pattern**: `**/patterns.yaml`

   - **File path pattern**: `**/conventions.yaml`

#### Editor Settings


1. Go to **File** → **Settings** → **Editor** → **Code Style** → **YAML**

2. Set **Indent** to **2 spaces**

3. Enable **Use tab character** if preferred

4. Go to **File** → **Settings** → **Editor** → **General** → **Auto Import**

5. Enable **Optimize imports on the fly**

6. Enable **Add unambiguous imports on the fly**

### 3. Configure External Tools


Set up external tools for Rhema CLI commands:

#### Rhema Initialize


1. Go to **File** → **Settings** → **Tools** → **External Tools**

2. Click **+** to add a new tool:

   - **Name**: `Rhema: Initialize Scope`

   - **Program**: `rhema`

   - **Arguments**: `init`

   - **Working directory**: `$ProjectFileDir$`

#### Rhema Validate


1. Add another external tool:

   - **Name**: `Rhema: Validate All`

   - **Program**: `rhema`

   - **Arguments**: `validate --recursive`

   - **Working directory**: `$ProjectFileDir$`

#### Rhema Health


1. Add another external tool:

   - **Name**: `Rhema: Show Health`

   - **Program**: `rhema`

   - **Arguments**: `health`

   - **Working directory**: `$ProjectFileDir$`

#### Rhema List Scopes


1. Add another external tool:

   - **Name**: `Rhema: List Scopes`

   - **Program**: `rhema`

   - **Arguments**: `scopes`

   - **Working directory**: `$ProjectFileDir$`

### 4. Configure Run Configurations


Create run configurations for common Rhema operations:

#### Rhema CLI Help


1. Go to **Run** → **Edit Configurations**

2. Click **+** → **Shell Script**

3. Configure:

   - **Name**: `Rhema Help`

   - **Script path**: `rhema`

   - **Script options**: `--help`

   - **Working directory**: `$ProjectFileDir$`

#### Rhema Query


1. Create another Shell Script configuration:

   - **Name**: `Rhema Query`

   - **Script path**: `rhema`

   - **Script options**: `query "$Prompt$"`

   - **Working directory**: `$ProjectFileDir$`

## Workflow Integration


### 1. Initialize a Rhema Scope


1. Open your project in IntelliJ

2. Go to **Tools** → **External Tools** → **Rhema: Initialize Scope**

3. Or use the terminal: `rhema init`

This creates the initial `.rhema/` directory with template files.

### 2. Configure AI Context


If using GitHub Copilot, create a `.copilot` file in your project root:

```
# Rhema Context Integration


This project uses Rhema (Git-Based Agent Context Protocol) for structured context management.

## Key Files to Reference:


- .rhema/rhema.yaml - Scope definition and metadata

- .rhema/knowledge.yaml - Domain knowledge and insights  

- .rhema/todos.yaml - Work items and tasks

- .rhema/decisions.yaml - Architecture decisions

- .rhema/patterns.yaml - Design patterns

- .rhema/conventions.yaml - Coding standards

## When Providing Assistance:


1. Check .rhema/knowledge.yaml for existing insights and domain knowledge

2. Review .rhema/decisions.yaml for architectural decisions

3. Consider .rhema/patterns.yaml for established design patterns

4. Follow .rhema/conventions.yaml for coding standards

5. Update relevant Rhema files when making significant changes

## Common Rhema Commands:


- rhema query "todos WHERE status='in_progress'" - Find active work

- rhema insight record "finding" - Record new insights

- rhema decision record "title" - Record architectural decisions

- rhema validate --recursive - Validate all Rhema files
```

### 3. Create Live Templates


Set up live templates for Rhema file creation:

#### Rhema Todo Template


1. Go to **File** → **Settings** → **Editor** → **Live Templates**

2. Create a new template group called **Rhema**

3. Add a new template:

   - **Abbreviation**: `rhema-todo`

   - **Description**: `Create a new Rhema todo item`

   - **Template text**:
```yaml

- id: "todo-$ID$"
  title: "$TITLE$"
  description: "$DESCRIPTION$"
  status: $STATUS$
  priority: $PRIORITY$
  assigned_to: "$ASSIGNEE$"
  created_at: "$DATE$"
  tags: [$TAGS$]
  related_components: [$COMPONENTS$]
```

   - **Variables**:

     - `$ID$`: `completeSmart()`

     - `$TITLE$`: `completeSmart()`

     - `$DESCRIPTION$`: `completeSmart()`

     - `$STATUS$`: `enum("pending", "in_progress", "completed", "blocked")`

     - `$PRIORITY$`: `enum("low", "medium", "high", "critical")`

     - `$ASSIGNEE$`: `completeSmart()`

     - `$DATE$`: `date("yyyy-MM-dd'T'HH:mm:ss'Z'")`

     - `$TAGS$`: `completeSmart()`

     - `$COMPONENTS$`: `completeSmart()`

#### Rhema Insight Template


1. Add another template:

   - **Abbreviation**: `rhema-insight`

   - **Description**: `Record a new Rhema insight`

   - **Template text**:
```yaml

- finding: "$FINDING$"
  impact: "$IMPACT$"
  solution: "$SOLUTION$"
  confidence: $CONFIDENCE$
  evidence: [$EVIDENCE$]
  related_files: [$FILES$]
  category: $CATEGORY$
  recorded_at: "$DATE$"
```

   - **Variables**:

     - `$FINDING$`: `completeSmart()`

     - `$IMPACT$`: `completeSmart()`

     - `$SOLUTION$`: `completeSmart()`

     - `$CONFIDENCE$`: `enum("low", "medium", "high")`

     - `$EVIDENCE$`: `completeSmart()`

     - `$FILES$`: `completeSmart()`

     - `$CATEGORY$`: `enum("performance", "security", "architecture", "user_experience")`

     - `$DATE$`: `date("yyyy-MM-dd'T'HH:mm:ss'Z'")`

## Git Integration


### 1. Git Configuration


IntelliJ has excellent built-in Git support:

#### Git Settings


1. Go to **File** → **Settings** → **Version Control** → **Git**

2. Ensure Git executable is properly configured

3. Configure Git user information if not already set

#### Git Toolbar


1. Enable the Git toolbar: **View** → **Appearance** → **Toolbar**

2. Customize Git toolbar buttons as needed

#### Git Branches


1. Use **Git** → **Branches** for branch management

2. Use **Git** → **Log** for commit history

3. Use **Git** → **Show History** for file history

### 2. Git Hooks Setup


Configure Git hooks for Rhema validation:

#### Pre-commit Hook


Create a `.git/hooks/pre-commit` file:

```bash
#!/bin/sh


# Rhema Pre-commit Hook


echo "Running Rhema validation..."

# Run Rhema validation


if command -v rhema >/dev/null 2>&1; then
    if ! rhema validate --recursive; then
        echo "Rhema validation failed. Please fix issues before committing."
        exit 1
    fi
    echo "Rhema validation passed."
else
    echo "Rhema CLI not found. Skipping validation."
fi
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## AI-Powered Workflows


### 1. Context-Aware Code Generation


When using GitHub Copilot or other AI assistants:

1. **Reference existing context**: "Based on the patterns in `.rhema/patterns.yaml`, generate..."

2. **Follow established decisions**: "Following the decision in `.rhema/decisions.yaml` about database choice..."

3. **Consider existing insights**: "Given the performance insights in `.rhema/knowledge.yaml`..."

### 2. Automated Context Updates


Use AI to help maintain Rhema files:

- "Update `.rhema/knowledge.yaml` with insights from this code change"

- "Record this architectural decision in `.rhema/decisions.yaml`"

- "Add a todo item for this technical debt in `.rhema/todos.yaml`"

### 3. Cross-Scope Analysis


For multi-scope projects:

- "Analyze the impact of this change across all Rhema scopes"

- "Find todos related to this feature across the entire project"

- "Identify knowledge gaps in the current Rhema context"

## Keyboard Shortcuts


Configure keyboard shortcuts for Rhema operations:

1. Go to **File** → **Settings** → **Keymap**

2. Search for **External Tools**

3. Assign shortcuts:

   - `Ctrl+Shift+G I` for **Rhema: Initialize Scope**

   - `Ctrl+Shift+G V` for **Rhema: Validate All**

   - `Ctrl+Shift+G H` for **Rhema: Show Health**

   - `Ctrl+Shift+G S` for **Rhema: List Scopes**

## Best Practices


### 1. Regular Context Maintenance


- Run `rhema validate --recursive` before commits

- Update knowledge files when discovering new insights

- Record decisions as they're made, not after the fact

- Keep todos current and accurate

### 2. AI Collaboration


- Use GitHub Copilot to help maintain Rhema files

- Ask AI to suggest context updates based on code changes

- Leverage AI to find relevant existing context

- Use AI to help identify knowledge gaps

### 3. Team Coordination


- Commit Rhema files with related code changes

- Use Rhema context in code reviews

- Share insights and decisions through Rhema files

- Use cross-scope queries for project-wide coordination

### 4. IntelliJ Specific


- Use the integrated terminal for Rhema commands

- Leverage IntelliJ's powerful Git integration

- Use live templates for consistent Rhema file creation

- Take advantage of IntelliJ's refactoring tools

## Troubleshooting


### Common Issues


1. **YAML validation errors**: Ensure your Rhema files follow the schema in `schemas/rhema.json`

2. **Missing context**: Run `rhema health` to check scope completeness

3. **AI not using context**: Verify `.copilot` file is properly configured

4. **Schema not loading**: Check that `schemas/rhema.json` path is correct in settings

5. **External tools not working**: Ensure Rhema CLI is in your PATH

### Getting Help


- Run `rhema --help` for command documentation

- Check the [Rhema README](../README.md) for protocol details

- Use `rhema validate --recursive` to identify issues

- Review the [protocol schemas](../schemas/) for file formats

- Check IntelliJ's built-in help and documentation

## Next Steps


1. **Initialize your first scope**: `rhema init`

2. **Explore existing context**: `rhema scopes` and `rhema query`

3. **Start recording knowledge**: Use `rhema insight record`

4. **Set up team workflows**: Share Rhema practices with your team

5. **Integrate with CI/CD**: Add Rhema validation to your build pipeline

For more advanced usage, see the [Rhema CLI Reference](../README.md#cli-command-reference), [Protocol Documentation](../schemas/), and [Rust Development Setup](../development/rust-setup.md). 