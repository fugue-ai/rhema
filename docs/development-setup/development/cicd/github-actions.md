# GitHub Actions Setup for GACP

This guide will help you set up comprehensive CI/CD workflows using GitHub Actions for GACP development. These workflows ensure code quality, security, and reliable deployments.

## Prerequisites

- GitHub repository with GACP project
- GitHub Actions enabled
- [GACP CLI](../README.md#installation) for local testing
- [Rust toolchain](rust-setup.md) for development

## Basic CI Workflow

### 1. Main CI Pipeline

Create `.github/workflows/ci.yml`:

```yaml
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    
    strategy:
      matrix:
        rust: [stable, beta, nightly]
        include:
          - rust: stable
            cache-key: stable
          - rust: beta
            cache-key: beta
          - rust: nightly
            cache-key: nightly
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust toolchain
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt, clippy
        override: true
        
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ matrix.cache-key }}-${{ hashFiles('**/Cargo.lock') }}
        restore-keys: |
          ${{ runner.os }}-cargo-${{ matrix.cache-key }}-
          
    - name: Check formatting
      run: cargo fmt -- --check
      
    - name: Run clippy
      run: cargo clippy -- -D warnings
      
    - name: Run tests
      run: cargo test --verbose
      
    - name: Run integration tests
      run: cargo test --test '*'
      
    - name: Build release
      run: cargo build --release
      
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: rhema-${{ matrix.rust }}
        path: target/release/rhema
        retention-days: 7

  rhema-validation:
    name: GACP Validation
    runs-on: ubuntu-latest
    needs: test
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Build GACP CLI
      run: cargo build --release
      
    - name: Install GACP CLI
      run: cargo install --path .
      
    - name: Validate GACP files
      run: rhema validate --recursive
      
    - name: Check GACP health
      run: rhema health
      
    - name: List GACP scopes
      run: rhema scopes

  security:
    name: Security Audit
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run security audit
      run: cargo audit
      
    - name: Check for secrets
      uses: trufflesecurity/trufflehog@main
      with:
        path: .
        base: ${{ github.event.before }}
        head: ${{ github.sha }}
```

### 2. Code Quality Workflow

Create `.github/workflows/code-quality.yml`:

```yaml
name: Code Quality

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  coverage:
    name: Code Coverage
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Install cargo-tarpaulin
      run: cargo install cargo-tarpaulin
      
    - name: Generate coverage report
      run: cargo tarpaulin --out Html --out Xml
      
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: ./target/tarpaulin/tarpaulin-report.xml
        flags: unittests
        name: codecov-umbrella
        fail_ci_if_error: true
        
    - name: Upload coverage artifact
      uses: actions/upload-artifact@v3
      with:
        name: coverage-report
        path: target/tarpaulin/tarpaulin-report.html
        retention-days: 30

  linting:
    name: Linting
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run clippy with all checks
      run: cargo clippy --all-targets --all-features -- -D warnings
      
    - name: Check for unused dependencies
      run: cargo udeps
      
    - name: Check for outdated dependencies
      run: cargo outdated --exit-code 1

  documentation:
    name: Documentation
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Generate documentation
      run: cargo doc --no-deps --document-private-items
      
    - name: Check documentation links
      run: cargo doc --no-deps --document-private-items --open
      
    - name: Upload documentation
      uses: actions/upload-artifact@v3
      with:
        name: documentation
        path: target/doc/
        retention-days: 30
```

## Advanced CI Workflows

### 1. Performance Testing

Create `.github/workflows/performance.yml`:

```yaml
name: Performance Testing

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]
  schedule:
    - cron: '0 2 * * 0'  # Weekly on Sunday at 2 AM

jobs:
  benchmark:
    name: Benchmark
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run benchmarks
      run: cargo bench
      
    - name: Generate benchmark report
      run: |
        cargo install cargo-criterion
        cargo criterion --message-format=json | tee benchmark-results.json
        
    - name: Upload benchmark results
      uses: actions/upload-artifact@v3
      with:
        name: benchmark-results
        path: benchmark-results.json
        retention-days: 90

  load-testing:
    name: Load Testing
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Build release version
      run: cargo build --release
      
    - name: Run load tests
      run: |
        # Create test data
        mkdir -p test-data
        for i in {1..100}; do
          echo "Creating test scope $i"
          mkdir -p "test-data/scope-$i/.rhema"
          cp .github/schemas/example.scope.yaml "test-data/scope-$i/.rhema/rhema.yaml"
        done
        
        # Run performance test
        time target/release/rhema query "*/todos WHERE status='active'"
```

### 2. Cross-Platform Testing

Create `.github/workflows/cross-platform.yml`:

```yaml
name: Cross-Platform Testing

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  test-linux:
    name: Linux
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run tests
      run: cargo test --verbose

  test-macos:
    name: macOS
    runs-on: macos-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run tests
      run: cargo test --verbose

  test-windows:
    name: Windows
    runs-on: windows-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Run tests
      run: cargo test --verbose
```

## Release Workflows

### 1. Release Pipeline

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    name: Build Release
    runs-on: ubuntu-latest
    
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            asset_name: rhema-linux-x86_64
          - target: x86_64-apple-darwin
            os: macos-latest
            asset_name: rhema-macos-x86_64
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            asset_name: rhema-windows-x86_64.exe
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: ${{ matrix.target }}
        
    - name: Build release
      run: cargo build --release --target ${{ matrix.target }}
      
    - name: Upload artifact
      uses: actions/upload-artifact@v3
      with:
        name: ${{ matrix.asset_name }}
        path: target/${{ matrix.target }}/release/rhema*
        retention-days: 30

  release:
    name: Create Release
    runs-on: ubuntu-latest
    needs: build
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Download all artifacts
      uses: actions/download-artifact@v3
      
    - name: Create Release
      uses: softprops/action-gh-release@v1
      with:
        files: |
          rhema-linux-x86_64/*
          rhema-macos-x86_64/*
          rhema-windows-x86_64.exe/*
        generate_release_notes: true
        draft: false
        prerelease: false
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 2. Crate Publishing

Create `.github/workflows/publish.yml`:

```yaml
name: Publish to Crates.io

on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    name: Publish
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Publish to crates.io
      run: cargo publish
      env:
        CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

## Deployment Workflows

### 1. Documentation Deployment

Create `.github/workflows/deploy-docs.yml`:

```yaml
name: Deploy Documentation

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  deploy:
    name: Deploy
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Generate documentation
      run: cargo doc --no-deps --document-private-items
      
    - name: Deploy to GitHub Pages
      uses: peaceiris/actions-gh-pages@v3
      if: github.ref == 'refs/heads/main'
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./target/doc
```

### 2. Docker Image Building

Create `.github/workflows/docker.yml`:

```yaml
name: Docker

on:
  push:
    branches: [ main ]
    tags:
      - 'v*'

jobs:
  docker:
    name: Build and Push Docker Image
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v2
      
    - name: Login to Docker Hub
      uses: docker/login-action@v2
      with:
        username: ${{ secrets.DOCKER_USERNAME }}
        password: ${{ secrets.DOCKER_PASSWORD }}
        
    - name: Build and push
      uses: docker/build-push-action@v4
      with:
        context: .
        push: true
        tags: |
          fugueai/rhema:latest
          fugueai/rhema:${{ github.sha }}
          fugueai/rhema:${{ github.ref_name }}
        cache-from: type=gha
        cache-to: type=gha,mode=max
```

## Security Workflows

### 1. Security Scanning

Create `.github/workflows/security.yml`:

```yaml
name: Security

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]
  schedule:
    - cron: '0 0 * * 0'  # Weekly

jobs:
  security-scan:
    name: Security Scan
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      
    - name: Run Trivy vulnerability scanner
      uses: aquasecurity/trivy-action@master
      with:
        scan-type: 'fs'
        scan-ref: '.'
        format: 'sarif'
        output: 'trivy-results.sarif'
        
    - name: Upload Trivy scan results to GitHub Security tab
      uses: github/codeql-action/upload-sarif@v2
      if: always()
      with:
        sarif_file: 'trivy-results.sarif'
        
    - name: Run OWASP ZAP scan
      uses: zaproxy/action-full-scan@v0.7.0
      with:
        target: 'https://your-app-url.com'
        
    - name: Run Bandit security linter
      uses: python-security/bandit-action@v1.0.0
      with:
        path: .
        level: low
```

## Monitoring and Notifications

### 1. Status Notifications

Create `.github/workflows/notifications.yml`:

```yaml
name: Notifications

on:
  workflow_run:
    workflows: ["CI", "Release"]
    types:
      - completed

jobs:
  notify:
    name: Notify
    runs-on: ubuntu-latest
    if: ${{ github.event.workflow_run.conclusion != 'skipped' }}
    
    steps:
    - name: Notify Slack
      uses: 8398a7/action-slack@v3
      with:
        status: ${{ github.event.workflow_run.conclusion }}
        channel: '#rhema-dev'
        webhook_url: ${{ secrets.SLACK_WEBHOOK }}
        
    - name: Notify Discord
      uses: sarisia/actions-status-discord@v1
      with:
        webhook: ${{ secrets.DISCORD_WEBHOOK }}
        status: ${{ github.event.workflow_run.conclusion }}
        title: GACP CI/CD Pipeline
```

## Configuration Files

### 1. Dependabot Configuration

Create `.github/dependabot.yml`:

```yaml
version: 2
updates:
  # Enable version updates for Rust
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
      time: "09:00"
    open-pull-requests-limit: 10
    reviewers:
      - "fugue-ai/rhema-maintainers"
    assignees:
      - "fugue-ai/rhema-maintainers"
    commit-message:
      prefix: "chore"
      include: "scope"
      
  # Enable version updates for GitHub Actions
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
      time: "09:00"
    open-pull-requests-limit: 5
    reviewers:
      - "fugue-ai/rhema-maintainers"
```

### 2. Issue Templates

Create `.github/ISSUE_TEMPLATE/bug_report.md`:

```markdown
---
name: Bug report
about: Create a report to help us improve
title: ''
labels: 'bug'
assignees: ''

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Run command '...'
2. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Environment:**
 - OS: [e.g. Ubuntu 20.04]
 - Rust version: [e.g. 1.70.0]
 - GACP version: [e.g. 1.2.0]

**Additional context**
Add any other context about the problem here.
```

## Troubleshooting

### Common Issues

1. **Cache misses**: Check cache key configuration
2. **Timeout issues**: Increase job timeout limits
3. **Permission errors**: Check repository secrets and permissions
4. **Cross-platform issues**: Test on multiple platforms
5. **Dependency conflicts**: Use `cargo update` and check `Cargo.lock`

### Best Practices

1. **Use caching**: Cache dependencies and build artifacts
2. **Parallel jobs**: Run independent jobs in parallel
3. **Fail fast**: Stop on first failure to save resources
4. **Security**: Use secrets for sensitive data
5. **Monitoring**: Set up notifications for failures

## Next Steps

1. **Set up secrets**: Configure required secrets in repository settings
2. **Test workflows**: Push changes to trigger workflow testing
3. **Monitor performance**: Track workflow execution times
4. **Optimize**: Refine workflows based on usage patterns
5. **Document**: Keep workflow documentation updated

For more information, see the [Git Workflow Setup](git-setup.md) and [Rust Development Setup](rust-setup.md) guides. 