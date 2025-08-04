# Rhema Examples

This directory contains comprehensive examples demonstrating various features and capabilities of the Rhema framework.

## Directory Structure

### 📁 [core/](./core/)
Basic functionality examples and fundamental Rhema features.

### 📁 [integration/](./integration/)
Examples showing how to integrate Rhema with external services and protocols:
- **gRPC Coordination**: Multi-agent coordination using gRPC
- **MCP Integration**: Model Context Protocol authentication and daemon examples
- **Syneidesis Integration**: Advanced coordination patterns
- **Coordination Testing**: Integration testing examples

### 📁 [advanced/](./advanced/)
Complex features and advanced workflows:
- **Agent Implementation**: Custom agent development
- **Conflict Prevention**: Advanced conflict resolution strategies
- **AI Service**: Lock file awareness and production features
- **Task Scoring**: Agentic development workflows
- **Version Management**: Automated version control
- **Query Engine**: Advanced querying capabilities
- **Dashboard**: Monitoring and visualization

### 📁 [config/](./config/)
Configuration examples and validation:
- **Action Intent**: YAML configuration for action intents
- **Validation Rules**: Custom validation rule definitions
- **Comprehensive Validation**: Advanced validation scenarios

### 📁 [testing/](./testing/)
Testing and validation examples:
- **Unit Tests**: Core functionality testing
- **Validation Tests**: Configuration validation testing

### 📁 [automation/](./automation/)
Git automation and workflow examples:
- **Branch Automation**: Feature, hotfix, and release branch workflows
- **Git Hooks**: Monitoring and automation hooks
- **Context-Aware Automation**: Intelligent automation based on context

## Getting Started

1. **Start with Core Examples**: Begin with the `core/` directory for fundamental concepts
2. **Explore Integration**: Check `integration/` for external service connections
3. **Advanced Features**: Dive into `advanced/` for complex workflows
4. **Configuration**: Review `config/` for setup and validation patterns
5. **Testing**: Use `testing/` examples for validation and testing strategies
6. **Automation**: Implement `automation/` examples for Git workflows

## Running Examples

Most examples can be run with:

```bash
cargo run --example <example_name>
```

For configuration examples, refer to the YAML files and run the corresponding Rust examples.

## Contributing

When adding new examples:
1. Place them in the appropriate category directory
2. Update this README with a brief description
3. Ensure examples are self-contained and well-documented
4. Include both simple and complex use cases where appropriate 