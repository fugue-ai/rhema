# MkDocs Migration Guide

This document outlines the migration of the Rhema documentation from a simple Markdown structure to a full-featured MkDocs site.

## 🎯 Migration Overview

The documentation has been converted from a basic directory structure with README files to a modern, searchable documentation site using:

- **MkDocs** - Static site generator
- **Material for MkDocs** - Modern, responsive theme
- **GitHub Pages** - Automated deployment
- **GitHub Actions** - CI/CD pipeline

## 📁 File Structure Changes

### Before (Old Structure)
```
docs/
├── README.md (main documentation index)
├── getting-started/
│   ├── README.md
│   ├── quick-start.md
│   ├── WORKSPACE_QUICK_START.md
│   └── REFACTORING_TO_WORKSPACE.md
├── user-guide/
│   ├── README.md
│   ├── cli-command-reference.md
│   └── ...
├── architecture/
│   ├── README.md
│   ├── proposals/
│   └── mcp/
└── ...
```

### After (MkDocs Structure)
```
├── mkdocs.yml (configuration)
├── requirements-docs.txt (dependencies)
├── docs/
│   ├── index.md (homepage)
│   ├── stylesheets/extra.css (custom styles)
│   ├── javascripts/mathjax.js (math support)
│   ├── getting-started/
│   │   ├── quick-start.md
│   │   ├── workspace-quick-start.md (renamed)
│   │   └── refactoring-to-workspace.md (renamed)
│   ├── user-guide/
│   ├── architecture/
│   └── ...
├── scripts/docs.sh (development helper)
└── .github/workflows/docs.yml (deployment)
```

## 🔧 Key Features Added

### 1. **Modern Theme**
- Material Design theme with dark/light mode toggle
- Responsive design for mobile and desktop
- Professional typography and spacing

### 2. **Enhanced Navigation**
- Hierarchical navigation structure
- Breadcrumb navigation
- Table of contents on each page
- Search functionality with suggestions

### 3. **Code Documentation**
- Syntax highlighting for multiple languages
- Copy-to-clipboard buttons
- Line numbers and annotations
- Code block folding

### 4. **Developer Experience**
- Live reload during development
- Git revision dates on pages
- Link validation
- Build optimization

### 5. **Deployment Automation**
- GitHub Actions workflow for CI/CD
- Automatic deployment to GitHub Pages
- Build validation on pull requests

## 🚀 Getting Started

### Local Development

1. **Install dependencies:**
   ```bash
   ./scripts/docs.sh install
   ```

2. **Start development server:**
   ```bash
   ./scripts/docs.sh serve
   ```

3. **Open browser:**
   Navigate to `http://127.0.0.1:8000`

### Production Build

```bash
./scripts/docs.sh build
```

This creates a `site/` directory with static HTML files ready for deployment.

## 📝 Configuration

### Main Configuration (`mkdocs.yml`)

Key configuration sections:

- **Site metadata** - Name, description, author
- **Theme settings** - Colors, features, navigation
- **Plugin configuration** - Search, git dates, minification
- **Markdown extensions** - Admonitions, code highlighting, math

### Custom Styling (`docs/stylesheets/extra.css`)

Custom CSS for:
- Code block styling
- Table formatting
- Navigation hover effects
- Mobile responsiveness

### JavaScript (`docs/javascripts/mathjax.js`)

MathJax configuration for mathematical notation support.

## 🔄 Migration Process

### 1. **File Renaming**
- `WORKSPACE_QUICK_START.md` → `workspace-quick-start.md`
- `REFACTORING_TO_WORKSPACE.md` → `refactoring-to-workspace.md`

### 2. **Navigation Structure**
- Created hierarchical navigation in `mkdocs.yml`
- Organized content into logical sections
- Added proper page titles and descriptions

### 3. **Link Updates**
- Updated internal links to match new file structure
- Fixed relative paths for navigation
- Added proper anchor links

### 4. **Theme Customization**
- Configured Material theme with Rhema branding
- Added custom CSS for improved styling
- Set up dark/light mode toggle

### 5. **Deployment Setup**
- Created GitHub Actions workflow
- Configured GitHub Pages deployment
- Added build validation

## 🛠️ Development Workflow

### Adding New Pages

1. Create the Markdown file in the appropriate directory
2. Add the page to the navigation in `mkdocs.yml`
3. Test locally with `./scripts/docs.sh serve`
4. Commit and push - automatic deployment will handle the rest

### Updating Existing Pages

1. Edit the Markdown file
2. Test changes locally
3. Commit and push

### Custom Styling

1. Edit `docs/stylesheets/extra.css`
2. Test changes in development server
3. Commit and push

## 🔍 Search and Navigation

### Search Features
- Full-text search across all pages
- Search suggestions
- Result highlighting
- Keyboard shortcuts

### Navigation Features
- Hierarchical menu structure
- Breadcrumb navigation
- Previous/next page links
- Table of contents

## 📱 Mobile Support

The documentation is fully responsive with:
- Mobile-optimized navigation
- Touch-friendly interface
- Readable typography on small screens
- Optimized images and code blocks

## 🚀 Deployment

### GitHub Pages
- Automatic deployment on push to `main` branch
- Build artifacts uploaded to GitHub Pages
- Custom domain support (if configured)

### Manual Deployment
```bash
./scripts/docs.sh build
# Upload site/ directory to your web server
```

## 🔧 Troubleshooting

### Common Issues

1. **Build errors:**
   - Check file paths in `mkdocs.yml`
   - Validate Markdown syntax
   - Ensure all referenced files exist

2. **Missing dependencies:**
   ```bash
   ./scripts/docs.sh install
   ```

3. **Broken links:**
   - Run `./scripts/docs.sh validate`
   - Check relative paths in Markdown files

4. **Styling issues:**
   - Clear browser cache
   - Check CSS syntax in `extra.css`

### Development Tips

- Use `./scripts/docs.sh serve` for live development
- Check the browser console for JavaScript errors
- Validate links before committing
- Test on both desktop and mobile

## 📚 Additional Resources

- [MkDocs Documentation](https://www.mkdocs.org/)
- [Material for MkDocs](https://squidfunk.github.io/mkdocs-material/)
- [PyMdown Extensions](https://facelessuser.github.io/pymdown-extensions/)
- [GitHub Pages](https://pages.github.com/)

## 🤝 Contributing

When contributing to the documentation:

1. Follow the existing structure and formatting
2. Test your changes locally before submitting
3. Update the navigation if adding new pages
4. Ensure all links are valid
5. Use clear, concise language with examples

## 📈 Future Enhancements

Potential improvements for the future:

- [ ] Add versioning support for different releases
- [ ] Integrate with API documentation generators
- [ ] Add interactive examples and demos
- [ ] Implement analytics and usage tracking
- [ ] Add multi-language support
- [ ] Create PDF export functionality 