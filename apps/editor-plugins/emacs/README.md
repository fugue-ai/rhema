# Rhema Emacs Integration

[![MELPA](https://melpa.org/packages/rhema-badge.svg)](https://melpa.org/#/rhema)
[![MELPA Stable](https://stable.melpa.org/packages/rhema-badge.svg)](https://stable.melpa.org/#/rhema)

Comprehensive Emacs integration for the Rhema Git-Based Agent Context Protocol system. This package provides full IDE support for working with Rhema files, including syntax highlighting, IntelliSense, validation, and interactive command execution.

## Features

### 🎯 Core Features
- **Syntax Highlighting**: Custom syntax highlighting for Rhema YAML files
- **IntelliSense**: Context-aware completion and suggestions
- **Real-time Validation**: Live error checking and feedback
- **Interactive Commands**: Execute all Rhema commands directly from Emacs
- **Git Integration**: Version control integration with conflict handling
- **Performance Monitoring**: Built-in performance profiling and caching

### 🚀 Advanced Features
- **Context Management**: Smart context detection and switching
- **Auto-validation**: Automatic file validation on save
- **Caching System**: Intelligent caching with TTL for performance
- **Error Handling**: Comprehensive error recovery and user feedback
- **Documentation**: Integrated help system and documentation
- **Customization**: Extensive configuration options

### 📋 Commands Available
- `rhema-command` - Execute any Rhema command interactively
- `rhema-show-context` - Show current Rhema context
- `rhema-validate` - Validate current Rhema file
- `rhema-show-scopes` - Show available Rhema scopes
- `rhema-show-tree` - Show Rhema scope tree
- `rhema-manage-todos` - Manage Rhema todos
- `rhema-manage-insights` - Manage Rhema insights
- `rhema-manage-patterns` - Manage Rhema patterns
- `rhema-manage-decisions` - Manage Rhema decisions
- `rhema-show-dependencies` - Show Rhema dependencies
- `rhema-show-impact` - Show Rhema impact analysis
- `rhema-sync-knowledge` - Sync Rhema knowledge
- `rhema-git-integration` - Show Git integration status
- `rhema-show-stats` - Show Rhema statistics
- `rhema-check-health` - Check Rhema health
- `rhema-debug-context` - Debug Rhema context
- `rhema-profile-performance` - Profile Rhema performance
- `rhema-refactor-context` - Refactor Rhema context
- `rhema-generate-code` - Generate code using Rhema
- `rhema-show-documentation` - Show Rhema documentation
- `rhema-configure-settings` - Configure Rhema settings
- `rhema-show-sidebar` - Show Rhema sidebar
- `rhema-status` - Show Rhema status
- `rhema-cache-clear` - Clear Rhema cache
- `rhema-cache-stats` - Show cache statistics

## Installation

### Using MELPA (Recommended)

Add MELPA to your package archives:

```elisp
(require 'package)
(add-to-list 'package-archives
             '("melpa" . "https://melpa.org/packages/") t)
(package-initialize)
```

Then install the package:

```elisp
M-x package-install RET rhema RET
```

### Using use-package

```elisp
(use-package rhema
  :ensure t
  :config
  (rhema-mode 1))
```

### Manual Installation

1. Clone the repository:
```bash
git clone https://github.com/fugue-ai/rhema.git
```

2. Add to your load path:
```elisp
(add-to-list 'load-path "~/path/to/rhema/apps/editor-plugins/emacs")
```

3. Require the package:
```elisp
(require 'rhema)
(rhema-mode 1)
```

## Configuration

### Basic Configuration

```elisp
;; Enable Rhema mode
(rhema-mode 1)

;; Set Rhema executable path (if not in PATH)
(setq rhema-executable "/path/to/rhema")

;; Enable auto-validation
(setq rhema-auto-validate t)

;; Enable IntelliSense
(setq rhema-intellisense t)

;; Enable Git integration
(setq rhema-git-integration t)

;; Enable debug mode (for verbose logging)
(setq rhema-debug-mode nil)

;; Enable performance profiling
(setq rhema-performance-profiling nil)
```

### Advanced Configuration

```elisp
;; Customize cache TTL (default: 300 seconds)
(setq rhema--cache-ttl 600)

;; Customize theme
(setq rhema-theme 'dark)

;; Customize language
(setq rhema-language "en")

;; Enable auto-sync with Git
(setq rhema-auto-sync t)

;; Enable context exploration
(setq rhema-context-exploration t)
```

## Usage

### Basic Usage

1. **Open a Rhema file**: Files with `.rhema.yml` extension will automatically use Rhema mode
2. **Execute commands**: Use `M-x rhema-command RET` to execute any Rhema command
3. **Show context**: Use `M-x rhema-show-context RET` to view current context
4. **Validate files**: Use `M-x rhema-validate RET` to validate current file

### Key Bindings

| Key Binding | Command | Description |
|-------------|---------|-------------|
| `C-c C-c` | `rhema-command` | Execute Rhema command |
| `C-c C-v` | `rhema-validate` | Validate current file |
| `C-c C-s` | `rhema-show-context` | Show current context |
| `C-c C-t` | `rhema-show-tree` | Show scope tree |
| `C-c C-d` | `rhema-manage-todos` | Manage todos |
| `C-c C-i` | `rhema-manage-insights` | Manage insights |
| `C-c C-p` | `rhema-manage-patterns` | Manage patterns |
| `C-c C-e` | `rhema-manage-decisions` | Manage decisions |
| `C-c C-g` | `rhema-git-integration` | Git integration |
| `C-c C-h` | `rhema-check-health` | Check health |
| `C-c C-r` | `rhema-refactor-context` | Refactor context |
| `C-c C-y` | `rhema-sync-knowledge` | Sync knowledge |
| `C-c C-u` | `rhema-cache-clear` | Clear cache |
| `C-c C-?` | `rhema-status` | Show status |

### Menu Integration

The package provides a comprehensive menu under "Rhema" with all available commands organized by category:

- **Core Commands**: Execute, context, validation, scopes, tree
- **Management**: Todos, insights, patterns, decisions
- **Analysis**: Dependencies, impact, sync, Git integration
- **Development**: Stats, health, debug, profile, refactor, generate
- **System**: Documentation, settings, sidebar, status, cache

### Completion

The package provides context-aware completion for Rhema files:

- **Omni-completion**: Use `C-M-i` or `M-TAB` for context-aware completions
- **Command completion**: Tab completion for Rhema commands
- **Value completion**: Intelligent suggestions based on context

### Validation

Real-time validation is available for Rhema files:

- **Auto-validation**: Files are automatically validated on save
- **Manual validation**: Use `M-x rhema-validate RET` for manual validation
- **Error highlighting**: Validation errors are highlighted in the buffer
- **Quick fixes**: Suggested fixes for common validation errors

## File Types

The package automatically detects and provides support for:

- `.rhema.yml` - Rhema YAML files
- `.rhema.yaml` - Rhema YAML files (alternative extension)
- Files containing `rhema:` in the first 1000 characters

## Performance

### Caching

The package includes an intelligent caching system:

- **Command results**: Cached for 5 minutes by default
- **Context information**: Cached to avoid repeated queries
- **Completions**: Cached for faster response times
- **Validation results**: Cached to improve performance

### Monitoring

Performance monitoring is available:

- **Command timing**: Track execution time for all commands
- **Cache statistics**: Monitor cache hit rates and usage
- **Memory usage**: Track memory consumption
- **Error tracking**: Monitor error rates and types

## Troubleshooting

### Common Issues

1. **Rhema executable not found**
   - Ensure `rhema` is in your PATH
   - Set `rhema-executable` to the correct path

2. **Commands not working**
   - Check if Rhema CLI is properly installed
   - Verify file permissions
   - Enable debug mode for verbose logging

3. **Completion not working**
   - Ensure `rhema-intellisense` is enabled
   - Check if file is recognized as Rhema file
   - Verify completion functions are loaded

4. **Validation errors**
   - Check Rhema file syntax
   - Verify schema compliance
   - Review error messages for specific issues

### Debug Mode

Enable debug mode for verbose logging:

```elisp
(setq rhema-debug-mode t)
```

This will show detailed information about:
- Command execution
- Cache operations
- File detection
- Error handling

### Getting Help

- **Documentation**: Use `M-x rhema-show-documentation RET`
- **Status**: Use `M-x rhema-status RET` to check system status
- **Health Check**: Use `M-x rhema-check-health RET` for diagnostics

## Development

### Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

### Testing

Run tests with:

```bash
cd apps/editor-plugins/emacs
emacs --batch --load rhema.el --eval "(ert-run-tests-batch-and-exit)"
```

### Building

To build the package:

```bash
cd apps/editor-plugins/emacs
emacs --batch --eval "(package-build-from-source)"
```

## License

This package is licensed under the Apache License, Version 2.0. See the LICENSE file for details.

## Support

- **Issues**: Report bugs and feature requests on GitHub
- **Documentation**: See the [Rhema documentation](https://rhema.ai/docs)
- **Community**: Join the [Rhema community](https://rhema.ai/community)

## Changelog

### Version 0.1.0
- Initial release
- Complete Rhema integration
- Syntax highlighting
- IntelliSense support
- Validation system
- Git integration
- Performance optimization
- Comprehensive documentation

---

**Rhema Emacs Integration** - Professional IDE support for the Rhema Git-Based Agent Context Protocol 