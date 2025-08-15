# Rhema Vim Plugin

A comprehensive Vim plugin for the Rhema Git-Based Agent Context Protocol system. This plugin provides syntax highlighting, completion, validation, and interactive commands for managing Rhema files and contexts.

## Features

### Core Functionality
- **Syntax Highlighting**: Full syntax highlighting for Rhema YAML files
- **Context Detection**: Automatic detection of Rhema files and project context
- **Command Integration**: Interactive commands for all Rhema CLI operations
- **File Validation**: Real-time validation with error highlighting
- **Auto-completion**: Context-aware completion for Rhema keywords and commands

### Advanced Features
- **Sidebar**: Project exploration with file navigation
- **Caching System**: Intelligent caching for improved performance
- **Performance Monitoring**: Built-in performance profiling
- **Git Integration**: Seamless Git workflow integration
- **Error Handling**: Comprehensive error handling and recovery
- **Template System**: Quick insertion of Rhema templates

### UI Features
- **Output Buffers**: Dedicated buffers for command output with navigation
- **Interactive Prompts**: User-friendly prompts for command options
- **Status Messages**: Clear success, error, warning, and info messages
- **Search Integration**: Search within output buffers and files

## Installation

### Manual Installation

1. Copy the plugin files to your Vim runtime directory:
   ```bash
   cp -r apps/editor-plugins/vim/* ~/.vim/
   ```

2. Add to your `.vimrc`:
   ```vim
   " Enable Rhema plugin
   let g:rhema_enabled = 1
   ```

### Plugin Manager Installation

#### Using vim-plug:
```vim
Plug 'your-repo/rhema', { 'rtp': 'apps/editor-plugins/vim' }
```

#### Using Vundle:
```vim
Plugin 'your-repo/rhema'
```

#### Using Pathogen:
```bash
git clone https://github.com/your-repo/rhema.git ~/.vim/bundle/rhema
```

## Requirements

- Vim 8.0+ or Neovim
- Rhema CLI executable in PATH
- Unix-like system (Linux, macOS)

## Configuration

### Global Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `g:rhema_enabled` | `1` | Enable/disable the plugin |
| `g:rhema_executable` | `'rhema'` | Path to Rhema CLI executable |
| `g:rhema_auto_validate` | `1` | Auto-validate files on save |
| `g:rhema_show_notifications` | `1` | Show notification messages |
| `g:rhema_intellisense` | `1` | Enable IntelliSense features |
| `g:rhema_debug_mode` | `0` | Enable debug logging |
| `g:rhema_performance_profiling` | `0` | Enable performance profiling |
| `g:rhema_context_exploration` | `1` | Enable context exploration |
| `g:rhema_git_integration` | `1` | Enable Git integration |
| `g:rhema_auto_sync` | `0` | Auto-sync with Git |
| `g:rhema_theme` | `'auto'` | UI theme |
| `g:rhema_language` | `'en'` | Language for messages |
| `g:rhema_timeout` | `30` | Command timeout in seconds |
| `g:rhema_cache_enabled` | `1` | Enable caching |
| `g:rhema_cache_ttl` | `300` | Cache TTL in seconds |
| `g:rhema_no_mappings` | `0` | Disable key mappings |
| `g:rhema_auto_format` | `0` | Auto-format files on save |
| `g:rhema_show_status` | `0` | Show status in statusline |

### Example Configuration

```vim
" Rhema plugin configuration
let g:rhema_enabled = 1
let g:rhema_executable = 'rhema'
let g:rhema_auto_validate = 1
let g:rhema_show_notifications = 1
let g:rhema_intellisense = 1
let g:rhema_debug_mode = 0
let g:rhema_performance_profiling = 0
let g:rhema_context_exploration = 1
let g:rhema_git_integration = 1
let g:rhema_auto_sync = 0
let g:rhema_theme = 'auto'
let g:rhema_language = 'en'
let g:rhema_timeout = 30
let g:rhema_cache_enabled = 1
let g:rhema_cache_ttl = 300
let g:rhema_no_mappings = 0
let g:rhema_auto_format = 0
let g:rhema_show_status = 0
```

## Commands

### Basic Commands

| Command | Description |
|---------|-------------|
| `:RhemaInitialize` | Initialize a new Rhema scope |
| `:RhemaShowContext` | Show current Rhema context |
| `:RhemaExecuteQuery` | Execute a CQL query |
| `:RhemaSearchContext` | Search in Rhema context |
| `:RhemaValidateFiles` | Validate Rhema files |
| `:RhemaShowScopes` | Show available scopes |
| `:RhemaShowTree` | Show scope tree |

### Management Commands

| Command | Description |
|---------|-------------|
| `:RhemaManageTodos` | Manage todos (add/list/complete/update/delete) |
| `:RhemaManageInsights` | Manage insights (record/list/update/delete) |
| `:RhemaManagePatterns` | Manage patterns (add/list/update/delete) |
| `:RhemaManageDecisions` | Manage decisions (record/list/update/delete) |

### Advanced Commands

| Command | Description |
|---------|-------------|
| `:RhemaShowDependencies` | Show dependencies |
| `:RhemaShowImpact` | Show impact analysis |
| `:RhemaSyncKnowledge` | Sync knowledge |
| `:RhemaGitIntegration` | Git integration |
| `:RhemaShowStats` | Show statistics |
| `:RhemaCheckHealth` | Check health |
| `:RhemaDebugContext` | Debug context |
| `:RhemaProfilePerformance` | Profile performance |
| `:RhemaRefactorContext` | Refactor context |
| `:RhemaGenerateCode` | Generate code |
| `:RhemaShowDocumentation` | Show documentation |
| `:RhemaConfigureSettings` | Configure settings |
| `:RhemaShowSidebar` | Show sidebar |
| `:RhemaStatus` | Show plugin status |
| `:RhemaCacheClear` | Clear cache |
| `:RhemaCacheStats` | Show cache statistics |

## Key Mappings

### Global Mappings (with leader key)

| Mapping | Command | Description |
|---------|---------|-------------|
| `<leader>gi` | `RhemaInitialize` | Initialize scope |
| `<leader>gc` | `RhemaShowContext` | Show context |
| `<leader>gq` | `RhemaExecuteQuery` | Execute query |
| `<leader>gs` | `RhemaSearchContext` | Search context |
| `<leader>gv` | `RhemaValidateFiles` | Validate files |
| `<leader>gp` | `RhemaShowScopes` | Show scopes |
| `<leader>gt` | `RhemaShowTree` | Show tree |
| `<leader>gt` | `RhemaManageTodos` | Manage todos |
| `<leader>gi` | `RhemaManageInsights` | Manage insights |
| `<leader>gp` | `RhemaManagePatterns` | Manage patterns |
| `<leader>gd` | `RhemaManageDecisions` | Manage decisions |
| `<leader>gd` | `RhemaShowDependencies` | Show dependencies |
| `<leader>gi` | `RhemaShowImpact` | Show impact |
| `<leader>gk` | `RhemaSyncKnowledge` | Sync knowledge |
| `<leader>gg` | `RhemaGitIntegration` | Git integration |
| `<leader>gs` | `RhemaShowStats` | Show stats |
| `<leader>gh` | `RhemaCheckHealth` | Check health |
| `<leader>gb` | `RhemaDebugContext` | Debug context |
| `<leader>gf` | `RhemaProfilePerformance` | Profile performance |
| `<leader>gr` | `RhemaRefactorContext` | Refactor context |
| `<leader>gc` | `RhemaGenerateCode` | Generate code |
| `<leader>gh` | `RhemaShowDocumentation` | Show documentation |
| `<leader>gc` | `RhemaConfigureSettings` | Configure settings |
| `<leader>gs` | `RhemaShowSidebar` | Show sidebar |
| `<leader>gs` | `RhemaStatus` | Show status |
| `<leader>gc` | `RhemaCacheClear` | Clear cache |
| `<leader>gc` | `RhemaCacheStats` | Show cache stats |

### Buffer-local Mappings (in Rhema files)

| Mapping | Description |
|---------|-------------|
| `<leader>v` | Validate current file |
| `<leader>c` | Show current context |
| `<leader>s` | Search in current file |
| `<leader>f` | Format current file |
| `<leader>i` | Show file statistics |
| `<leader>n` | Next related file |
| `<leader>p` | Previous related file |
| `<leader>tt` | Insert todo template |
| `<leader>ti` | Insert insight template |
| `<leader>tp` | Insert pattern template |
| `<leader>td` | Insert decision template |

### Output Buffer Mappings

| Mapping | Description |
|---------|-------------|
| `q` | Close buffer |
| `<CR>` | Open selected item |
| `/` | Search in output |
| `n` | Next search result |
| `N` | Previous search result |
| `y` | Copy current line |
| `Y` | Copy all output |
| `r` | Refresh output |
| `?` | Show help |

### Sidebar Mappings

| Mapping | Description |
|---------|-------------|
| `q` | Close sidebar |
| `<CR>` | Open selected file |
| `r` | Refresh sidebar |
| `?` | Show help |

## File Type Support

The plugin automatically detects and provides support for the following file types:

- `*.rhema.yml`
- `rhema.yml`
- `scope.yaml`
- `knowledge.yaml`
- `todos.yaml`
- `decisions.yaml`
- `patterns.yaml`
- `conventions.yaml`

### Features for Rhema files:
- Syntax highlighting
- Auto-indentation
- Completion
- Validation
- Error highlighting
- Quick navigation
- Template insertion

## Completion

The plugin provides context-aware completion for Rhema files:

### Omni-completion (Ctrl-X Ctrl-O):
- Keywords
- Commands
- File paths
- Values based on key context

### Command-line completion:
- Rhema commands
- File names
- Options

### Template completion:
- Todo templates
- Insight templates
- Pattern templates
- Decision templates

## UI Features

### Output Buffers
- Dedicated buffers for command output
- Syntax highlighting
- Search functionality
- Copy/paste support
- Navigation between items

### Sidebar
- Project context overview
- File navigation
- Quick access to scopes
- Recent items

### Interactive Prompts
- Command-line prompts for options
- Confirmation dialogs
- Selection menus
- Progress indicators

### Status Messages
- Success messages
- Error messages
- Warning messages
- Info messages

## Examples

### Basic Usage

1. Initialize a new scope:
   ```vim
   :RhemaInitialize
   ```

2. Show current context:
   ```vim
   :RhemaShowContext
   ```

3. Execute a query:
   ```vim
   :RhemaExecuteQuery
   ```

4. Validate files:
   ```vim
   :RhemaValidateFiles
   ```

5. Manage todos:
   ```vim
   :RhemaManageTodos
   ```

### Advanced Usage

1. Show sidebar:
   ```vim
   :RhemaShowSidebar
   ```

2. Check plugin status:
   ```vim
   :RhemaStatus
   ```

3. Clear cache:
   ```vim
   :RhemaCacheClear
   ```

4. Profile performance:
   ```vim
   :RhemaProfilePerformance
   ```

5. Debug context:
   ```vim
   :RhemaDebugContext
   ```

### File Editing

1. Open a Rhema file:
   ```vim
   :edit scope.yaml
   ```

2. Use completion:
   - Press `Ctrl-X Ctrl-O` for omni-completion
   - Type `:` and use command-line completion

3. Insert templates:
   - `<leader>tt` for todo template
   - `<leader>ti` for insight template
   - `<leader>tp` for pattern template
   - `<leader>td` for decision template

4. Validate current file:
   ```vim
   <leader>v
   ```

## Testing

Run the test suite:

```vim
:RhemaTest
```

Run specific tests:

```vim
:RhemaTestSpecific initialization
:RhemaTestSpecific configuration
:RhemaTestSpecific file_detection
:RhemaTestSpecific completion
:RhemaTestSpecific validation
:RhemaTestSpecific ui
:RhemaTestSpecific cache
:RhemaTestSpecific performance
```

## Troubleshooting

### Common Issues

1. **"Rhema executable not found"**
   - Solution: Ensure rhema CLI is installed and in PATH

2. **"Command failed"**
   - Solution: Check rhema CLI installation and permissions

3. **"No completion"**
   - Solution: Ensure filetype is set to 'rhema'

4. **"No syntax highlighting"**
   - Solution: Check if syntax is enabled and filetype is correct

5. **"Plugin not loading"**
   - Solution: Check Vim version (8.0+) and plugin installation

### Debug Mode

Enable debug mode to see detailed logs:

```vim
let g:rhema_debug_mode = 1
```

### Performance Issues

- Disable performance profiling if not needed
- Clear cache if memory usage is high
- Check timeout settings

## Development

### Project Structure

```
apps/editor-plugins/vim/
├── plugin/
│   └── gacp.vim              # Main plugin file
├── autoload/
│   ├── rhema.vim             # Main autoload file
│   └── rhema/
│       ├── commands.vim      # Command implementations
│       ├── complete.vim      # Completion system
│       └── ui.vim           # UI system
├── syntax/
│   └── rhema.vim             # Syntax highlighting
├── ftplugin/
│   └── rhema.vim             # File type specific settings
├── doc/
│   └── rhema.txt            # Documentation
├── test/
│   └── rhema_test.vim       # Test suite
└── README.md                # This file
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

Copyright 2025 Cory Parent

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Support

For more information, visit: https://github.com/your-repo/rhema

For issues and feature requests, please use the GitHub issue tracker. 