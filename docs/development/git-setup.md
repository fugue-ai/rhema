# Git Workflow Setup for GACP Development

This guide will help you set up an effective Git workflow for GACP development. Since GACP is Git-native, proper Git setup and workflows are crucial for maintaining context integrity and enabling effective collaboration.

## Prerequisites

- [Git](https://git-scm.com/) installed and configured
- [GACP CLI](../README.md#installation) installed
- Basic familiarity with Git concepts

## Git Configuration

### 1. Global Git Configuration

Set up your Git identity and preferences:

```bash
# Set your identity
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# Set default branch name
git config --global init.defaultBranch main

# Configure line endings
git config --global core.autocrlf input  # On macOS/Linux
git config --global core.autocrlf true   # On Windows

# Configure merge strategy
git config --global merge.ff false
git config --global pull.rebase true

# Configure push behavior
git config --global push.default current
git config --global push.autoSetupRemote true

# Configure credential helper
git config --global credential.helper cache
git config --global credential.helper 'cache --timeout=3600'
```

### 2. Repository-Specific Configuration

Configure Git settings specific to GACP development:

```bash
# Navigate to your GACP repository
cd gacp

# Set up repository-specific config
git config core.hooksPath .git/hooks
git config core.editor "code --wait"  # or your preferred editor
git config core.excludesfile .gitignore

# Configure merge strategy for YAML files
git config merge.ours.driver true
```

## Branching Strategy

### 1. Branch Naming Conventions

Follow these conventions for branch names:

```bash
# Feature branches
git checkout -b feature/gacp-query-enhancement
git checkout -b feature/add-yaml-validation

# Bug fix branches
git checkout -b fix/gacp-parse-error
git checkout -b fix/memory-leak-in-query

# Documentation branches
git checkout -b docs/update-readme
git checkout -b docs/add-api-examples

# Release branches
git checkout -b release/v1.2.0
git checkout -b hotfix/critical-security-fix
```

### 2. Branch Protection Rules

Set up branch protection for the main branch:

```bash
# Create a protection script
cat > scripts/setup-branch-protection.sh << 'EOF'
#!/bin/bash
# This script sets up branch protection rules via GitHub API

REPO="fugue-ai/gacp"
BRANCH="main"

# Requires GitHub CLI or curl with appropriate authentication
gh api repos/$REPO/branches/$BRANCH/protection \
  --method PUT \
  --field required_status_checks='{"strict":true,"contexts":["ci/tests","ci/lint","ci/security"]}' \
  --field enforce_admins=true \
  --field required_pull_request_reviews='{"required_approving_review_count":2,"dismiss_stale_reviews":true}' \
  --field restrictions=null
EOF

chmod +x scripts/setup-branch-protection.sh
```

## Commit Conventions

### 1. Conventional Commits

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```bash
# Format: <type>[optional scope]: <description>

# Feature commits
git commit -m "feat: add support for cross-scope queries"
git commit -m "feat(cli): implement new gacp query command"

# Bug fixes
git commit -m "fix: resolve YAML parsing error in gacp.yaml"
git commit -m "fix(schema): correct validation for nested objects"

# Documentation
git commit -m "docs: update README with installation instructions"
git commit -m "docs(api): add examples for query language"

# Performance improvements
git commit -m "perf: optimize query parsing for large files"
git commit -m "perf(schema): improve validation performance"

# Breaking changes
git commit -m "feat!: change query syntax to use SQL-like format"
git commit -m "BREAKING CHANGE: rename gacp.yaml to scope.yaml"
```

### 2. Commit Message Template

Create a commit message template:

```bash
# Create commit template
cat > .gitmessage << 'EOF'
# <type>(<scope>): <subject>
#
# <body>
#
# <footer>

# Types:
#   feat:     A new feature
#   fix:      A bug fix
#   docs:     Documentation only changes
#   style:    Changes that do not affect the meaning of the code
#   refactor: A code change that neither fixes a bug nor adds a feature
#   perf:     A code change that improves performance
#   test:     Adding missing tests or correcting existing tests
#   chore:    Changes to the build process or auxiliary tools

# Scopes:
#   cli:      Command-line interface
#   schema:   Protocol schema definitions
#   query:    Query language implementation
#   git:      Git integration
#   scope:    Scope management
#   test:     Testing infrastructure

# Examples:
#   feat(cli): add new gacp query command
#   fix(schema): correct validation for nested objects
#   docs: update README with installation guide
EOF

# Set the template
git config commit.template .gitmessage
```

## Git Hooks

### 1. Pre-commit Hook

Create a comprehensive pre-commit hook:

```bash
#!/bin/sh
# .git/hooks/pre-commit

echo "Running pre-commit checks..."

# Check for GACP CLI
if ! command -v gacp >/dev/null 2>&1; then
    echo "Warning: GACP CLI not found. Skipping GACP validation."
else
    # Run GACP validation
    echo "Running GACP validation..."
    if ! gacp validate --recursive; then
        echo "GACP validation failed. Please fix issues before committing."
        exit 1
    fi
    echo "GACP validation passed."
fi

# Check for Rust toolchain (if this is a Rust project)
if command -v cargo >/dev/null 2>&1; then
    # Format check
    echo "Checking code formatting..."
    if ! cargo fmt -- --check; then
        echo "Code formatting check failed. Run 'cargo fmt' to fix."
        exit 1
    fi

    # Clippy check
    echo "Running clippy..."
    if ! cargo clippy -- -D warnings; then
        echo "Clippy check failed. Fix warnings before committing."
        exit 1
    fi

    # Test check
    echo "Running tests..."
    if ! cargo test; then
        echo "Tests failed. Fix tests before committing."
        exit 1
    fi
fi

# Check for large files
echo "Checking for large files..."
if git diff --cached --name-only | xargs -I {} sh -c 'if [ -f "{}" ]; then size=$(stat -f%z "{}" 2>/dev/null || stat -c%s "{}" 2>/dev/null || echo 0); if [ "$size" -gt 10485760 ]; then echo "Large file detected: {} ($size bytes)"; exit 1; fi; fi'; then
    echo "Large file check passed."
else
    echo "Large file check failed. Consider using Git LFS for large files."
    exit 1
fi

# Check for merge conflicts
echo "Checking for merge conflicts..."
if git grep -l "<<<<<<< HEAD" -- '*.rs' '*.yaml' '*.md' '*.toml'; then
    echo "Merge conflicts detected. Please resolve before committing."
    exit 1
fi

echo "All pre-commit checks passed!"
```

### 2. Commit-msg Hook

Create a commit message validation hook:

```bash
#!/bin/sh
# .git/hooks/commit-msg

# Conventional Commits validation
commit_regex='^(feat|fix|docs|style|refactor|perf|test|chore)(\(.+\))?: .{1,50}'

if ! grep -qE "$commit_regex" "$1"; then
    echo "Invalid commit message format."
    echo "Please follow the Conventional Commits specification:"
    echo "  <type>[optional scope]: <description>"
    echo ""
    echo "Types: feat, fix, docs, style, refactor, perf, test, chore"
    echo "Examples:"
    echo "  feat: add new query feature"
    echo "  fix(cli): resolve parsing error"
    echo "  docs: update README"
    exit 1
fi
```

### 3. Pre-push Hook

Create a pre-push validation hook:

```bash
#!/bin/sh
# .git/hooks/pre-push

echo "Running pre-push checks..."

# Run full test suite
if command -v cargo >/dev/null 2>&1; then
    echo "Running full test suite..."
    if ! cargo test --all-features; then
        echo "Test suite failed. Please fix before pushing."
        exit 1
    fi
fi

# Run security audit
if command -v cargo >/dev/null 2>&1; then
    echo "Running security audit..."
    if ! cargo audit; then
        echo "Security audit failed. Please address vulnerabilities."
        exit 1
    fi
fi

# Check for GACP validation
if command -v gacp >/dev/null 2>&1; then
    echo "Running GACP validation..."
    if ! gacp validate --recursive; then
        echo "GACP validation failed. Please fix before pushing."
        exit 1
    fi
fi

echo "All pre-push checks passed!"
```

### 4. Post-merge Hook

Create a post-merge hook for dependency updates:

```bash
#!/bin/sh
# .git/hooks/post-merge

echo "Running post-merge tasks..."

# Update dependencies if Cargo.lock changed
if git diff-tree -r --name-only --no-commit-id HEAD | grep -q "Cargo.lock"; then
    echo "Cargo.lock changed, updating dependencies..."
    cargo update
fi

# Regenerate documentation if needed
if git diff-tree -r --name-only --no-commit-id HEAD | grep -q "src/.*\.rs"; then
    echo "Source files changed, regenerating documentation..."
    cargo doc --no-deps
fi

echo "Post-merge tasks completed."
```

## Workflow Automation

### 1. Git Aliases

Set up useful Git aliases:

```bash
# Add to your .gitconfig or run these commands

# Basic workflow aliases
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.st status

# Advanced aliases
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
git config --global alias.visual '!gitk'

# GACP-specific aliases
git config --global alias.gacp-init '!gacp init'
git config --global alias.gacp-validate '!gacp validate --recursive'
git config --global alias.gacp-health '!gacp health'
git config --global alias.gacp-scopes '!gacp scopes'

# Workflow aliases
git config --global alias.feature '!git checkout -b feature/'
git config --global alias.fix '!git checkout -b fix/'
git config --global alias.docs '!git checkout -b docs/'
git config --global alias.release '!git checkout -b release/'

# Cleanup aliases
git config --global alias.cleanup '!git branch --merged | grep -v "\\*" | xargs -n 1 git branch -d'
git config --global alias.prune-remote '!git remote prune origin'
```

### 2. Git Workflow Scripts

Create helper scripts for common workflows:

```bash
# scripts/feature.sh
#!/bin/bash
# Create a new feature branch

if [ -z "$1" ]; then
    echo "Usage: $0 <feature-name>"
    exit 1
fi

feature_name=$(echo "$1" | tr ' ' '-')
branch_name="feature/$feature_name"

echo "Creating feature branch: $branch_name"
git checkout -b "$branch_name"

echo "Feature branch created. Don't forget to:"
echo "1. Make your changes"
echo "2. Run tests: cargo test"
echo "3. Format code: cargo fmt"
echo "4. Commit with conventional format"
echo "5. Push and create PR"
```

```bash
# .github/scripts/release.sh
#!/bin/bash
# Create a release

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

version="$1"
release_branch="release/v$version"

echo "Creating release $version..."

# Create release branch
git checkout -b "$release_branch"

# Update version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"$version\"/" Cargo.toml

# Update CHANGELOG.md
echo "## [$version] - $(date +%Y-%m-%d)" >> CHANGELOG.md
echo "" >> CHANGELOG.md
echo "### Added" >> CHANGELOG.md
echo "### Changed" >> CHANGELOG.md
echo "### Fixed" >> CHANGELOG.md
echo "" >> CHANGELOG.md

# Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "chore: prepare release v$version"

echo "Release branch created: $release_branch"
echo "Next steps:"
echo "1. Review and test the release"
echo "2. Merge to main"
echo "3. Create git tag: git tag v$version"
echo "4. Push tag: git push origin v$version"
```

## Team Collaboration

### 1. Pull Request Workflow

Follow this workflow for contributions:

```bash
# 1. Create feature branch
git checkout -b feature/your-feature

# 2. Make changes and commit
git add .
git commit -m "feat: implement your feature"

# 3. Push and create PR
git push -u origin feature/your-feature

# 4. After review, squash commits
git rebase -i main

# 5. Merge via GitHub/GitLab interface
```

### 2. Code Review Guidelines

Create `.github/pull_request_template.md`:

```markdown
## Description

Brief description of changes made.

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing

- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed
- [ ] GACP validation passes

## Checklist

- [ ] Code follows the project's style guidelines
- [ ] Self-review of code completed
- [ ] Code is commented, particularly in hard-to-understand areas
- [ ] Corresponding changes to documentation made
- [ ] No new warnings generated
- [ ] Tests added that prove fix is effective or feature works

## GACP Context

- [ ] GACP files updated if needed
- [ ] Context changes documented
- [ ] Cross-scope impact considered
```

### 3. Branch Protection

Set up branch protection rules:

```yaml
# .github/branch-protection.yml
name: Branch Protection

on:
  push:
    branches: [ main, develop ]

jobs:
  protect:
    runs-on: ubuntu-latest
    steps:
    - name: Protect main branch
      uses: actions/github-script@v6
      with:
        script: |
          github.rest.repos.updateBranchProtection({
            owner: context.repo.owner,
            repo: context.repo.repo,
            branch: 'main',
            required_status_checks: {
              strict: true,
              contexts: ['ci/tests', 'ci/lint', 'ci/security']
            },
            enforce_admins: true,
            required_pull_request_reviews: {
              required_approving_review_count: 2,
              dismiss_stale_reviews: true
            },
            restrictions: null
          })
```

## Troubleshooting

### Common Issues

1. **Merge conflicts**: Use `git status` to see conflicts, resolve manually
2. **Large files**: Use Git LFS for files > 10MB
3. **Commit history issues**: Use `git rebase -i` to clean up history
4. **Branch protection**: Ensure you have proper permissions
5. **Hook failures**: Check hook permissions with `ls -la .git/hooks/`

### Getting Help

- [Git Documentation](https://git-scm.com/doc)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Flow](https://guides.github.com/introduction/flow/)
- [Git LFS](https://git-lfs.github.com/)

## Next Steps

1. **Set up your hooks**: Run the hook setup scripts
2. **Configure your editor**: See [Editor Setup Guides](../editor-setup/)
3. **Practice the workflow**: Make a small contribution
4. **Join the team**: Participate in code reviews
5. **Automate more**: Add custom scripts for your workflow

For more information, see the [GACP CLI Reference](../README.md#cli-command-reference) and [Contributing Guide](contributing.md). 