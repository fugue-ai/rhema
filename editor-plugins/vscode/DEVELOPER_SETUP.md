# VS Code Extension - Developer Setup Guide

## Overview
This guide provides step-by-step instructions for setting up the Rhema VS Code extension for development and testing. The extension is ready for testing with mock/placeholder implementations for backend features.

## 🚀 Quick Start

### Prerequisites
- **Node.js**: Version 16 or higher
- **VS Code**: Version 1.85.0 or higher
- **TypeScript**: Version 4.9 or higher
- **Git**: For version control and testing Git features

### Installation Steps

1. **Clone the Repository**
   ```bash
   git clone https://github.com/fugue-ai/rhema.git
   cd rhema/editor-plugins/vscode
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Compile the Extension**
   ```bash
   npm run compile
   ```

4. **Open in VS Code**
   ```bash
   code .
   ```

## 🔧 Development Setup

### 1. Extension Development Environment

#### Launch Configuration
Create `.vscode/launch.json` in the extension directory:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Run Extension",
      "type": "extensionHost",
      "request": "launch",
      "args": [
        "--extensionDevelopmentPath=${workspaceFolder}"
      ],
      "outFiles": [
        "${workspaceFolder}/out/**/*.js"
      ],
      "preLaunchTask": "npm: compile"
    },
    {
      "name": "Extension Tests",
      "type": "extensionHost",
      "request": "launch",
      "args": [
        "--extensionDevelopmentPath=${workspaceFolder}",
        "--extensionTestsPath=${workspaceFolder}/out/test/suite/index"
      ],
      "outFiles": [
        "${workspaceFolder}/out/test/**/*.js"
      ],
      "preLaunchTask": "npm: compile"
    }
  ]
}
```

#### Tasks Configuration
Create `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "type": "npm",
      "script": "compile",
      "group": "build",
      "presentation": {
        "panel": "shared",
        "reveal": "silent"
      },
      "problemMatcher": "$tsc"
    },
    {
      "type": "npm",
      "script": "watch",
      "group": "build",
      "presentation": {
        "panel": "shared",
        "reveal": "never"
      },
      "isBackground": true,
      "problemMatcher": "$tsc-watch"
    },
    {
      "type": "npm",
      "script": "test",
      "group": "test",
      "presentation": {
        "panel": "shared",
        "reveal": "always"
      }
    }
  ]
}
```

### 2. Testing Environment Setup

#### Test Workspace
Create a test workspace with sample Rhema files:

```bash
mkdir test-workspace
cd test-workspace
```

#### Sample Rhema Files
Create `test-workspace/rhema.yml`:

```yaml
scope:
  name: "test-scope"
  description: "Test scope for development"
  version: "1.0.0"
  author: "Developer"
  created: "2025-01-01"
  updated: "2025-01-01"
  tags: ["test", "development"]
  dependencies: []

context:
  files: ["src/**/*.rs", "docs/**/*.md"]
  patterns: ["*.rs", "*.md"]
  exclusions: ["target/**/*", "node_modules/**/*"]
  maxTokens: 10000
  includeHidden: false
  recursive: true

todos:
  - id: "todo-1"
    title: "Implement feature X"
    description: "Add new functionality to the system"
    priority: "high"
    status: "in-progress"
    assignee: "developer"
    dueDate: "2025-02-01"
    tags: ["feature", "implementation"]
    related: []

insights:
  - id: "insight-1"
    title: "Performance bottleneck found"
    description: "The system shows slow response times in certain scenarios"
    type: "performance"
    confidence: 0.85
    source: "analysis"
    tags: ["performance", "optimization"]
    related: []

patterns:
  - id: "pattern-1"
    name: "error-handling"
    description: "Standard error handling pattern"
    type: "code-pattern"
    confidence: 0.9
    source: "codebase"
    tags: ["error-handling", "best-practice"]
    related: []

decisions:
  - id: "decision-1"
    title: "Use Rust for backend"
    description: "Decision to use Rust for the backend implementation"
    type: "architecture"
    status: "approved"
    date: "2025-01-01"
    tags: ["architecture", "technology"]
    related: []
```

Create `test-workspace/scope.yml`:

```yaml
scope:
  name: "sub-scope"
  description: "Sub-scope for testing"
  version: "0.1.0"
  dependencies: ["../rhema.yml"]

context:
  files: ["lib/**/*.rs"]
  patterns: ["*.rs"]
  maxTokens: 5000
```

### 3. Mock Backend Setup

#### Placeholder Rhema CLI
Since the backend features are not yet implemented, create a mock Rhema CLI for testing:

Create `mock-rhema.js` in the extension directory:

```javascript
#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Mock Rhema CLI for testing
class MockRhemaCLI {
  constructor() {
    this.version = '0.1.0-mock';
  }

  async validate(args) {
    console.log('Mock: Validating Rhema files...');
    return { success: true, issues: [] };
  }

  async todos(args) {
    console.log('Mock: Checking todos...');
    return { success: true, todos: [] };
  }

  async decisions(args) {
    console.log('Mock: Checking decisions...');
    return { success: true, decisions: [] };
  }

  async context(args) {
    console.log('Mock: Analyzing context...');
    return { success: true, context: {} };
  }
}

const cli = new MockRhemaCLI();
const command = process.argv[2];
const args = process.argv.slice(3);

switch (command) {
  case '--version':
    console.log(cli.version);
    break;
  case 'validate':
    cli.validate(args);
    break;
  case 'todos':
    cli.todos(args);
    break;
  case 'decisions':
    cli.decisions(args);
    break;
  case 'context':
    cli.context(args);
    break;
  default:
    console.log('Mock Rhema CLI - Available commands: validate, todos, decisions, context');
}
```

Make it executable:
```bash
chmod +x mock-rhema.js
```

#### Update Extension Settings
Update the extension settings to use the mock CLI:

```json
{
  "rhema.executablePath": "./mock-rhema.js",
  "rhema.enabled": true,
  "rhema.aiCompletions": true,
  "rhema.autoValidate": true,
  "rhema.showNotifications": true,
  "rhema.intelliSense": true,
  "rhema.debugMode": true
}
```

## 🧪 Testing the Extension

### 1. Launch Extension in Development Mode

1. **Open the Extension Project**
   ```bash
   cd rhema/editor-plugins/vscode
   code .
   ```

2. **Press F5** or use the "Run Extension" launch configuration

3. **New VS Code Window Opens** with the extension loaded

### 2. Test Features

#### Test IntelliSense
1. Open the test workspace
2. Open `rhema.yml`
3. Start typing in different sections:
   - Type `scope:` and press Enter
   - Type `context:` and press Enter
   - Type `todos:` and press Enter
4. Verify completions appear

#### Test Validation
1. Open `rhema.yml`
2. Introduce errors (e.g., remove required fields)
3. Save the file
4. Check for validation errors in the Problems panel

#### Test Git Integration
1. Initialize a Git repository in the test workspace
2. Make changes to Rhema files
3. Check Git status integration
4. Test commit message templates

#### Test Commands
1. Open Command Palette (Ctrl+Shift+P)
2. Type "Rhema" to see available commands
3. Test each command:
   - `Rhema: Initialize Scope`
   - `Rhema: Show Context`
   - `Rhema: Execute Query`
   - `Rhema: Search Context`
   - `Rhema: Validate Files`

### 3. Debug Features

#### Enable Debug Mode
```json
{
  "rhema.debugMode": true
}
```

#### Check Output
1. Open Output panel (View → Output)
2. Select "RHEMA" from the dropdown
3. Monitor extension activity

#### Check Developer Tools
1. Help → Toggle Developer Tools
2. Check Console for any errors
3. Monitor network requests (when backend is ready)

## 🔍 Troubleshooting

### Common Issues

#### Extension Not Loading
1. Check VS Code version (requires 1.85.0+)
2. Verify TypeScript compilation succeeded
3. Check Output panel for errors
4. Restart VS Code

#### IntelliSense Not Working
1. Verify file is recognized as Rhema file
2. Check `rhema.intelliSense` setting is enabled
3. Reload the window (Ctrl+Shift+P → "Developer: Reload Window")

#### Validation Not Working
1. Check `rhema.autoValidate` setting is enabled
2. Verify file syntax is correct YAML
3. Check Problems panel for validation errors

#### Git Integration Issues
1. Ensure Git is installed and configured
2. Check `rhema.gitIntegration` setting is enabled
3. Verify workspace is a Git repository

### Debug Commands

#### Extension Status
```bash
# Check extension status
rhema check-health
```

#### Validate Configuration
```bash
# Validate extension configuration
rhema validate-config
```

#### Test Providers
```bash
# Test language service providers
npm run test:providers
```

## 📝 Development Workflow

### 1. Making Changes

1. **Edit Source Files**
   - Modify TypeScript files in `src/`
   - Changes are automatically compiled in watch mode

2. **Test Changes**
   - Press F5 to launch extension in new window
   - Test the specific feature you modified
   - Check for errors in Output panel

3. **Debug Issues**
   - Set breakpoints in TypeScript files
   - Use Developer Tools for debugging
   - Check extension logs

### 2. Adding New Features

1. **Create Feature Branch**
   ```bash
   git checkout -b feature/new-feature
   ```

2. **Implement Feature**
   - Add TypeScript code
   - Update package.json if needed
   - Add tests

3. **Test Feature**
   - Launch extension in development mode
   - Test with various scenarios
   - Check for edge cases

4. **Update Documentation**
   - Update this guide if needed
   - Add feature documentation
   - Update TODO.md

### 3. Testing Backend Integration

When backend features are ready:

1. **Update Executable Path**
   ```json
   {
     "rhema.executablePath": "/path/to/real/rhema"
   }
   ```

2. **Test Real Commands**
   - Test actual Rhema CLI commands
   - Verify AI completions work
   - Test real validation

3. **Performance Testing**
   - Test with large workspaces
   - Monitor memory usage
   - Check response times

## 🚀 Production Build

### Building for Distribution

1. **Compile for Production**
   ```bash
   npm run compile
   ```

2. **Package Extension**
   ```bash
   npm run package
   ```

3. **Install Locally**
   ```bash
   code --install-extension rhema-0.1.0.vsix
   ```

### Publishing to Marketplace

1. **Update Version**
   ```bash
   npm version patch
   ```

2. **Publish**
   ```bash
   npm run publish
   ```

## 📚 Additional Resources

### Documentation
- [VS Code Extension API](https://code.visualstudio.com/api)
- [TypeScript Documentation](https://www.typescriptlang.org/docs/)
- [Rhema Protocol Documentation](../../docs/)

### Testing
- [Extension Testing Guide](https://code.visualstudio.com/api/working-with-extensions/testing-extension)
- [VS Code Extension Samples](https://github.com/microsoft/vscode-extension-samples)

### Development Tools
- [VS Code Extension Generator](https://github.com/microsoft/vscode-generator-code)
- [Extension Pack](https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-extension-pack)

---

## 🎯 Next Steps

1. **Test Current Features**: Use this guide to test all implemented features
2. **Report Issues**: Document any bugs or missing functionality
3. **Backend Integration**: Follow SERVER_TODOS.md for backend implementation
4. **User Feedback**: Gather feedback from early users
5. **Performance Optimization**: Monitor and optimize performance

---

*Last Updated: January 2025*
*Extension Version: 0.1.0*
*Status: Ready for Testing* 