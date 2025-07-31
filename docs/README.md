# Rhema Documentation

This directory contains the documentation for the Rhema project, now built with [MkDocs](https://www.mkdocs.org/) and the [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/) theme.

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

The documentation is organized into the following sections:

- **Getting Started** - Essential guides for new users
- **User Guide** - Comprehensive feature documentation
- **Reference** - Technical reference materials
- **Development Setup** - Guides for contributors
- **Architecture** - Design decisions and proposals
- **Examples** - Practical use cases and examples

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

1. Follow the existing structure and formatting
2. Use clear, concise language with practical examples
3. Include code snippets where appropriate
4. Update the navigation in `mkdocs.yml` if adding new pages
5. Test your changes locally before submitting

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