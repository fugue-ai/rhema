# Rhema Interactive Mode


## Overview


The Rhema Interactive Mode provides a comprehensive REPL-style interface for managing Git-Based Agent Context Protocol (Rhema) repositories. It offers an intuitive, context-aware command-line experience with advanced features like auto-completion, syntax highlighting, command history, and plugin support.

## Features


### 🚀 Core Features


- **REPL-style Interface**: Interactive command execution with immediate feedback

- **Command History**: Persistent command history with navigation

- **Auto-completion**: Intelligent command and argument completion

- **Syntax Highlighting**: Colored output for better readability

- **Context-aware Suggestions**: Smart suggestions based on current context

- **Plugin System**: Extensible architecture with plugin support

### 🎯 Advanced Features


- **Multi-step Workflows**: Create and execute command workflows

- **Variable Management**: Set and use variables across sessions

- **Context Navigation**: Navigate between different scopes

- **Interactive Visualization**: Visual data exploration and analysis

- **Debug Tools**: Built-in debugging and profiling capabilities

- **Performance Monitoring**: Real-time performance tracking

### 🎨 User Experience


- **Customizable Themes**: Multiple theme options (Default, Dark, Light, Custom)

- **Configurable Prompts**: Customizable command prompts

- **Cross-platform Support**: Works on Windows, macOS, and Linux

- **Accessibility Features**: Support for screen readers and assistive technologies

## Getting Started


### Installation


The interactive mode is included with the Rhema CLI. No additional installation is required.

### Basic Usage


Start the interactive mode:

```bash
rhema interactive
```

Or with specific options:

```bash
rhema interactive --no-auto-complete --no-syntax-highlighting
```

### Configuration


Create a configuration file `~/.rhema/interactive.yaml`:

```yaml
prompt: "rhema> "
history_file: "~/.rhema_history"
max_history_size: 10000
auto_complete: true
syntax_highlighting: true
show_suggestions: true
context_aware: true
theme: Default
plugins:

  - context

  - visualization
keybindings: {}
```

## Commands


### Core Commands


| Command | Description | Example |
|---------|-------------|---------|
| `init` | Initialize a new Rhema scope | `init --scope-type service --scope-name api` |
| `scopes` | List all scopes in the repository | `scopes` |
| `scope` | Show scope details | `scope my-service` |
| `tree` | Show scope hierarchy tree | `tree` |
| `show` | Display YAML file content | `show knowledge --scope my-service` |
| `query` | Execute a CQL query | `query "SELECT * FROM scopes"` |
| `search` | Search across context files | `search "authentication" --regex` |

### Management Commands


| Command | Description | Example |
|---------|-------------|---------|
| `validate` | Validate YAML files | `validate --recursive` |
| `migrate` | Migrate schema files | `migrate --dry-run` |
| `schema` | Generate schema templates | `schema scope --output-file scope.yaml` |
| `health` | Check scope health | `health --scope my-service` |
| `stats` | Show context statistics | `stats` |

### Content Commands


| Command | Description | Example |
|---------|-------------|---------|
| `todo` | Manage todo items | `todo list --priority high` |
| `insight` | Manage knowledge insights | `insight record "API Design" --content "..."` |
| `pattern` | Manage patterns | `pattern add "Circuit Breaker" --description "..."` |
| `decision` | Manage decisions | `decision record "Database Choice" --description "..."` |

### Advanced Commands


| Command | Description | Example |
|---------|-------------|---------|
| `dependencies` | Show scope dependencies | `dependencies` |
| `impact` | Show impact of changes | `impact src/main.rs` |
| `sync` | Sync knowledge across scopes | `sync` |
| `git` | Advanced Git integration | `git status` |

### Export Commands


| Command | Description | Example |
|---------|-------------|---------|
| `export` | Export context data | `export --format json --include-all` |
| `primer` | Generate context primer files | `primer --scope-name my-service` |
| `readme` | Generate README with context | `readme --include-context` |
| `bootstrap` | Bootstrap context for AI agents | `bootstrap --use-case code_review` |

### Interactive Commands


| Command | Description | Example |
|---------|-------------|---------|
| `set` | Set a variable | `set my_var "value"` |
| `get` | Get a variable | `get my_var` |
| `workflow` | Manage workflows | `workflow create test "scopes" "stats"` |
| `plugin` | Manage plugins | `plugin list` |
| `visualize` | Interactive data visualization | `visualize scopes` |
| `debug` | Debug mode | `debug context` |
| `profile` | Performance profiling | `profile "scopes"` |
| `context` | Context management | `context navigate my-service` |
| `navigate` | Navigate between scopes | `navigate my-service` |
| `cache` | Cache management | `cache clear` |
| `explore` | Interactive context exploration | `explore` |

### System Commands


| Command | Description | Example |
|---------|-------------|---------|
| `help` | Show help | `help` |
| `clear` | Clear screen | `clear` |
| `history` | Show command history | `history` |
| `config` | Show configuration | `config` |
| `exit` | Exit interactive mode | `exit` |

## Advanced Features


### Workflows


Create and execute multi-step workflows:

```bash
# Create a workflow


workflow create daily-check "scopes" "health" "stats"

# Run a workflow


workflow run daily-check

# List workflows


workflow list
```

### Variables


Set and use variables across your session:

```bash
# Set variables


set project_name "my-api"
set scope_filter "service"

# Use variables in commands


query "SELECT * FROM scopes WHERE name LIKE '%${project_name}%'"
```

### Context Navigation


Navigate between different scopes:

```bash
# Navigate to a specific scope


navigate my-service

# Show current context


context

# Explore context interactively


explore
```

### Visualization


Visualize your data interactively:

```bash
# Visualize scopes


visualize scopes

# Visualize dependencies


visualize dependencies

# Visualize statistics


visualize stats
```

### Debugging


Use built-in debugging tools:

```bash
# Debug context


debug context

# Debug cache


debug cache

# Debug performance


debug performance
```

### Profiling


Profile command performance:

```bash
# Profile a command


profile "scopes --recursive"
```

## Configuration


### Configuration File


The interactive mode can be configured using a YAML file. By default, it looks for `~/.rhema/interactive.yaml`.

```yaml
# Basic configuration


prompt: "rhema> "
history_file: "~/.rhema_history"
max_history_size: 10000

# Feature toggles


auto_complete: true
syntax_highlighting: true
show_suggestions: true
context_aware: true

# Theme configuration


theme: Default
# theme: Dark


# theme: Light


# theme:


#   prompt_color: "cyan"


#   error_color: "red"


#   success_color: "green"


#   warning_color: "yellow"


#   info_color: "blue"


#   suggestion_color: "magenta"


# Plugin configuration


plugins:

  - context

  - visualization

# Keybindings (future feature)


keybindings: {}
```

### Command Line Options


| Option | Description | Default |
|--------|-------------|---------|
| `--config` | Configuration file path | `~/.rhema/interactive.yaml` |
| `--no-auto-complete` | Disable auto-completion | `false` |
| `--no-syntax-highlighting` | Disable syntax highlighting | `false` |
| `--no-context-aware` | Disable context-aware features | `false` |

## Plugins


### Built-in Plugins


#### Context Plugin


Provides context management and exploration features:

```bash
# Context commands


context explore
context navigate my-service
context cache
```

#### Visualization Plugin


Provides data visualization capabilities:

```bash
# Visualization commands


visualize scopes
visualize dependencies
visualize stats
```

### Creating Custom Plugins


Plugins can be created by implementing the `InteractivePlugin` trait:

```rust
use rhema::commands::interactive::InteractivePlugin;

pub struct MyPlugin;

impl InteractivePlugin for MyPlugin {
    fn name(&self) -> &str {
        "my_plugin"
    }
    
    fn description(&self) -> &str {
        "My custom plugin"
    }
    
    fn commands(&self) -> Vec<String> {
        vec!["my_command".to_string()]
    }
    
    fn execute(&self, session: &mut InteractiveSession, args: &[String]) -> RhemaResult<()> {
        // Plugin implementation
        Ok(())
    }
    
    fn suggestions(&self, session: &InteractiveSession, input: &str) -> Vec<String> {
        // Provide suggestions
        vec![]
    }
}
```

## Keyboard Shortcuts


| Shortcut | Description |
|----------|-------------|
| `Tab` | Auto-complete |
| `Ctrl+C` | Interrupt current command |
| `Ctrl+D` | Exit interactive mode |
| `Up/Down` | Navigate command history |
| `Ctrl+L` | Clear screen |
| `Ctrl+R` | Search command history |

## Themes


### Default Theme


The default theme provides a clean, professional appearance with cyan prompts and colored output.

### Dark Theme


Optimized for dark terminals with white text on black background.

### Light Theme


Optimized for light terminals with black text on white background.

### Custom Theme


Define your own color scheme:

```yaml
theme:
  prompt_color: "cyan"
  error_color: "red"
  success_color: "green"
  warning_color: "yellow"
  info_color: "blue"
  suggestion_color: "magenta"
```

## Performance


### Optimization Tips


1. **Use caching**: The interactive mode caches context data for faster access

2. **Limit history size**: Adjust `max_history_size` based on your needs

3. **Disable unused features**: Turn off features you don't use

4. **Use workflows**: Create workflows for common command sequences

### Performance Monitoring


Monitor performance using built-in tools:

```bash
# Profile command performance


profile "scopes --recursive"

# Debug performance


debug performance
```

## Troubleshooting


### Common Issues


#### Auto-completion not working


1. Check if auto-completion is enabled in configuration

2. Ensure you're using a compatible terminal

3. Try restarting the interactive mode

#### History not persisting


1. Check the `history_file` configuration

2. Ensure the directory exists and is writable

3. Check file permissions

#### Syntax highlighting issues


1. Verify your terminal supports colors

2. Check the `syntax_highlighting` configuration

3. Try a different theme

#### Plugin loading errors


1. Check plugin configuration

2. Verify plugin implementation

3. Check for missing dependencies

### Debug Mode


Use debug mode to diagnose issues:

```bash
# Debug context


debug context

# Debug cache


debug cache

# Debug performance


debug performance
```

### Logging


Enable verbose logging for debugging:

```bash
rhema interactive --verbose
```

## Examples


### Basic Workflow


```bash
# Start interactive mode


rhema interactive

# Initialize a new scope


init --scope-type service --scope-name my-api

# List scopes


scopes

# Navigate to the scope


navigate my-api

# Show scope details


scope

# Add some knowledge


show knowledge
# Edit the knowledge file manually, then continue


# Validate the scope


health

# Export context


export --format json --include-all
```

### Advanced Workflow


```bash
# Start with custom configuration


rhema interactive --config ~/.rhema/custom.yaml

# Set up variables


set project_name "my-microservice"
set environment "production"

# Create a workflow for daily checks


workflow create daily-check "scopes" "health" "stats" "dependencies"

# Run the workflow


workflow run daily-check

# Explore context interactively


explore

# Visualize dependencies


visualize dependencies

# Profile performance


profile "query 'SELECT * FROM scopes'"
```

### Plugin Development


```bash
# List available plugins


plugin list

# Get plugin information


plugin info context

# Use plugin commands


context explore
context navigate my-service
```

## Integration


### IDE Integration


The interactive mode can be integrated with IDEs through the MCP daemon:

```bash
# Start the MCP daemon


daemon start

# Use interactive mode with daemon


interactive
```

### CI/CD Integration


Use interactive mode in CI/CD pipelines:

```bash
# Non-interactive mode for automation


rhema scopes
rhema validate --recursive
rhema export --format json --include-all
```

### Scripting


The interactive mode can be scripted:

```bash
#!/bin/bash


echo "scopes" | rhema interactive
echo "health" | rhema interactive
echo "exit" | rhema interactive
```

## Best Practices


### Configuration Management


1. **Use version control**: Store configuration files in version control

2. **Environment-specific configs**: Use different configs for different environments

3. **Backup history**: Regularly backup your command history

### Workflow Design


1. **Keep workflows simple**: Focus on common, repeatable tasks

2. **Use variables**: Make workflows configurable with variables

3. **Document workflows**: Add comments to complex workflows

### Performance Optimization


1. **Cache frequently used data**: Use the built-in caching system

2. **Limit scope size**: Keep scopes focused and manageable

3. **Use appropriate commands**: Choose the most efficient commands for your needs

### Security


1. **Secure configuration**: Protect sensitive configuration data

2. **Audit history**: Regularly review command history

3. **Plugin security**: Only use trusted plugins

## Future Enhancements


### Planned Features


- **Advanced keybindings**: Custom keybinding support

- **Plugin marketplace**: Centralized plugin distribution

- **Cloud integration**: Cloud storage and synchronization

- **Collaborative features**: Multi-user support

- **Advanced visualization**: Interactive charts and graphs

- **AI assistance**: Intelligent command suggestions

### Contributing


To contribute to the interactive mode:

1. Fork the repository

2. Create a feature branch

3. Implement your changes

4. Add tests

5. Submit a pull request

## Support


### Documentation


- [Rhema Documentation](https://docs.rs/rhema)

- [Interactive Mode API Reference](https://docs.rs/rhema/interactive)

- [Plugin Development Guide](https://docs.rs/rhema/plugins)

### Community


- [GitHub Issues](https://github.com/fugue-ai/rhema/issues)

- [Discussions](https://github.com/fugue-ai/rhema/discussions)

- [Discord](https://discord.gg/rhema)

### Examples


- [Interactive Mode Examples](https://github.com/fugue-ai/rhema/examples/interactive)

- [Plugin Examples](https://github.com/fugue-ai/rhema/examples/plugins)

- [Configuration Examples](https://github.com/fugue-ai/rhema/examples/config) 