# Rhema VS Code Extension

A comprehensive VS Code extension for the Rhema Git-Based Agent Context Protocol, providing intelligent completions, validation, and Git integration.

## 🚀 Quick Start

### For Users
1. Install the extension from the VS Code Marketplace
2. Open a workspace with Rhema files
3. Start using intelligent completions and validation

### For Developers
```bash
# Quick setup
./setup-dev.sh

# Or manual setup
npm install
npm run compile
chmod +x mock-rhema.js
code .
# Press F5 to launch extension in development mode
```

📚 **Detailed setup instructions**: [DEVELOPER_SETUP.md](./DEVELOPER_SETUP.md)

## ✨ Features

### 🧠 AI-Powered Intelligent Completions
- Context-aware suggestions based on workspace state
- Intelligent completion based on document patterns
- Semantic search and error resolution suggestions
- Performance optimized with intelligent caching

### ✅ Complete Rhema Schema Validation
- Comprehensive validation against all Rhema schema types
- Custom validation rules and cross-reference validation
- Real-time validation with detailed error messages
- File existence checks and relationship validation

### 🔧 Advanced Git Workflow Features
- Branch management with naming conventions
- Rhema-specific commit message templates
- Conflict resolution strategies and Git hooks
- Automated workflow setup and management

### 🎯 Context-Aware Features
- Workspace analysis and context extraction
- Cross-reference support for existing components
- Real-time context updates
- File existence validation

## 🛠️ Development Status

### ✅ Completed (Client-Side)
- AI-powered intelligent completions
- Context-aware completion based on workspace state
- Complete Rhema-specific schema validation
- Advanced Git workflow features
- Performance optimizations
- Comprehensive testing infrastructure

### 🔄 In Progress (Server-Side)
- AI Integration Service
- Enhanced Validation Engine
- Git Integration Enhancements
- Context Management Service

📋 **Server-side TODOs**: [SERVER_TODOS.md](./SERVER_TODOS.md)

## 📁 Project Structure

```
editor-plugins/vscode/
├── src/
│   ├── providers/
│   │   ├── intelliSense.ts      # AI-powered completions
│   │   ├── validation.ts        # Schema validation
│   │   ├── rhemaProvider.ts     # Workspace context
│   │   └── gacpProvider.ts      # Language services
│   ├── gitIntegration.ts        # Git workflow features
│   ├── settings.ts              # Configuration management
│   └── extension.ts             # Main extension
├── test-workspace/              # Sample files for testing
├── mock-rhema.js               # Mock CLI for testing
├── setup-dev.sh                # Quick setup script
├── DEVELOPER_SETUP.md          # Detailed setup guide
├── SERVER_TODOS.md             # Backend requirements
└── TODO.md                     # Implementation tracking
```

## 🧪 Testing

### Test Workspace
The extension includes a test workspace with sample Rhema files:
- `test-workspace/rhema.yml` - Main scope file
- `test-workspace/scope.yml` - Sub-scope file

### Mock Backend
For testing without a full backend:
- `mock-rhema.js` - Mock Rhema CLI with all commands
- Simulates real CLI behavior for testing

## 🔧 Configuration

### Extension Settings
```json
{
  "rhema.enabled": true,
  "rhema.aiCompletions": true,
  "rhema.autoValidate": true,
  "rhema.gitIntegration": true,
  "rhema.debugMode": false
}
```

### Mock CLI Setup
```json
{
  "rhema.executablePath": "./mock-rhema.js"
}
```

## 📚 Documentation

- [Developer Setup Guide](./DEVELOPER_SETUP.md) - Complete setup instructions
- [Implementation Summary](./IMPLEMENTATION_SUMMARY.md) - Feature overview
- [Server-Side TODOs](./SERVER_TODOS.md) - Backend requirements
- [Implementation Tracking](./TODO.md) - Progress tracking

## 🚀 Getting Started

1. **Clone the repository**
   ```bash
   git clone https://github.com/fugue-ai/rhema.git
   cd rhema/editor-plugins/vscode
   ```

2. **Run setup script**
   ```bash
   ./setup-dev.sh
   ```

3. **Launch extension**
   ```bash
   code .
   # Press F5 to launch in development mode
   ```

4. **Test features**
   - Open test workspace
   - Try completions in Rhema files
   - Test validation and Git integration
   - Check command palette for Rhema commands

## 🤝 Contributing

1. Follow the [Developer Setup Guide](./DEVELOPER_SETUP.md)
2. Create a feature branch
3. Implement your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

Apache License 2.0 - see [LICENSE](../../LICENSE) for details.

---

**Status**: Client-side complete, ready for testing and backend integration
**Version**: 0.1.0
**Last Updated**: January 2025 