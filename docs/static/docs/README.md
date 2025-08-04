# Rhema Documentation

This directory contains the comprehensive documentation for the Rhema project, built with [MkDocs](https://www.mkdocs.org/) and the [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/) theme.

## 🚀 Quick Start

### Local Development

1. Install the documentation dependencies:
   ```bash
   pip install -r requirements-docs.txt
   ```

2. Start the development server:
   ```bash
   mkdocs serve
   ```

3. Open your browser and navigate to `http://127.0.0.1:8000`

### Building for Production

To build the documentation for production:

```bash
mkdocs build
```

This will create a `site/` directory with the static HTML files.

## 📁 Documentation Structure

The documentation is organized into the following logical sections:

### 🎯 Getting Started
Essential guides for new users to get up and running quickly
- Quick Start guides
- Workspace setup
- Basic concepts

### 📖 User Guide
Comprehensive feature documentation for end users
- CLI commands and usage
- Configuration management
- Interactive mode
- Performance monitoring
- Batch operations
- Conflict resolution

### 🔧 Core Features
Deep dives into Rhema's core functionality
- Lock file system
- AI integration
- Cache system
- Context injection
- CI/CD integration

### 📚 Reference
Technical reference materials and specifications
- Configuration schemas
- API documentation
- Schema examples

### 🏗️ Architecture
Design decisions, proposals, and technical architecture
- System design documents
- Architecture proposals
- MCP (Model Context Protocol) documentation

### 💡 Examples
Practical use cases and real-world examples
- Advanced usage patterns
- Template management
- Query examples
- Workflow demonstrations

### 🛠️ Development Setup
Guides for contributors and developers
- Local development setup
- Editor configuration
- CI/CD pipelines
- Contributing guidelines

## 🔧 Configuration

The documentation is configured via `mkdocs.yml` in the project root. Key features include:

- **Material for MkDocs theme** with dark/light mode toggle
- **Search functionality** with suggestions and highlighting
- **Code syntax highlighting** with copy buttons
- **Responsive design** for mobile and desktop
- **Git revision dates** for tracking document changes
- **Mathematical notation support** via MathJax

## 🚀 Deployment

The documentation is automatically deployed to GitHub Pages via GitHub Actions when changes are pushed to the `main` branch. The workflow is defined in `.github/workflows/docs.yml`.

## 📝 Contributing

When contributing to the documentation:

1. **Follow the structure**: Place new content in the appropriate section
2. **Use clear language**: Write concise, practical content with examples
3. **Include code snippets**: Where appropriate, provide working examples
4. **Update navigation**: Add new pages to the navigation in `mkdocs.yml`
5. **Test locally**: Verify your changes work before submitting
6. **Use consistent formatting**: Follow existing markdown patterns

### Content Guidelines

- **Getting Started**: Focus on quick wins and basic concepts
- **User Guide**: Comprehensive but accessible feature documentation
- **Core Features**: Technical deep-dives with implementation details
- **Reference**: Complete, accurate technical specifications
- **Architecture**: Design rationale and system overview
- **Examples**: Real-world, practical use cases
- **Development Setup**: Step-by-step contributor guides

## 🎨 Customization

### Styling

Custom CSS can be added to `docs/stylesheets/extra.css`. The current customizations include:

- Improved code block styling
- Better table formatting
- Enhanced navigation hover effects
- Mobile responsiveness improvements

### JavaScript

Custom JavaScript can be added to `docs/javascripts/`. Currently includes:

- MathJax configuration for mathematical notation

## 📚 Additional Resources

- [MkDocs Documentation](https://www.mkdocs.org/)
- [Material for MkDocs Documentation](https://squidfunk.github.io/mkdocs-material/)
- [PyMdown Extensions](https://facelessuser.github.io/pymdown-extensions/) 