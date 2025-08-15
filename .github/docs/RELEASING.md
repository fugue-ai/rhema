# Releasing Rhema CLI

This document describes the process for releasing new versions of Rhema CLI to Cargo and GitHub.

## Prerequisites

Before you can publish to Cargo, you need to:

1. **Create a Cargo account**: Visit [crates.io](https://crates.io) and create an account
2. **Get an API token**: Go to your account settings and generate a new API token
3. **Add the token to GitHub**: Add the token as a GitHub secret named `CARGO_REGISTRY_TOKEN`

### Setting up GitHub Secrets

1. Go to your GitHub repository settings
2. Navigate to "Secrets and variables" → "Actions"
3. Add a new repository secret:
   - Name: `CARGO_REGISTRY_TOKEN`
   - Value: Your Cargo API token

## Release Process

### 1. Prepare for Release

Before creating a release, ensure:

- [ ] All tests pass locally: `cargo test`
- [ ] Integration tests pass: `cargo test --test integration`
- [ ] Security audit passes: `cargo audit`
- [ ] Code is formatted: `cargo fmt`
- [ ] Clippy checks pass: `cargo clippy`
- [ ] Documentation is up to date
- [ ] README.md is current

### 2. Update Version

Update the version in `Cargo.toml`:

```toml
[package]
version = "0.1.1"  # Increment as appropriate
```

### 3. Update Changelog

Create or update a changelog file (e.g., `CHANGELOG.md`) with:

- New features
- Bug fixes
- Breaking changes
- Performance improvements
- Documentation updates

### 4. Create and Push Tag

Create a git tag for the release:

```bash
# Create an annotated tag
git tag -a v0.1.1 -m "Release v0.1.1"

# Push the tag
git push origin v0.1.1
```

**Important**: The tag must start with `v` (e.g., `v0.1.1`) to trigger the release workflow.

### 5. Automated Release Process

When you push a tag starting with `v`, the GitHub Actions workflow will automatically:

1. **Run Tests**: Execute all tests across multiple Rust versions
2. **Build Binaries**: Create release binaries for Linux, macOS, and Windows
3. **Publish to Cargo**: Upload the package to crates.io
4. **Create GitHub Release**: Create a GitHub release with downloadable binaries

### 6. Verify Release

After the workflow completes:

1. **Check Cargo**: Visit [crates.io/crates/rhema](https://crates.io/crates/rhema) to verify the new version
2. **Check GitHub**: Visit the releases page to verify the GitHub release was created
3. **Test Installation**: Try installing the new version:
   ```bash
   cargo install rhema
   ```

## Versioning Strategy

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

## Pre-release Testing

For pre-release testing, you can:

1. **Test locally**: `cargo publish --dry-run`
2. **Use a test crate**: Create a test crate on crates.io for testing the release process
3. **Use GitHub releases**: Create a draft release manually for testing

## Troubleshooting

### Common Issues

1. **Authentication Error**: Ensure `CARGO_REGISTRY_TOKEN` is set correctly
2. **Version Already Exists**: Increment the version number in `Cargo.toml`
3. **Build Failures**: Check the GitHub Actions logs for specific errors
4. **Missing Dependencies**: Ensure all dependencies are properly specified

### Manual Publishing

If the automated workflow fails, you can publish manually:

```bash
# Login to Cargo
cargo login

# Publish
cargo publish
```

## Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update changelog
- [ ] Run all tests locally
- [ ] Create and push git tag
- [ ] Monitor GitHub Actions workflow
- [ ] Verify release on crates.io
- [ ] Verify GitHub release
- [ ] Test installation from Cargo
- [ ] Update documentation if needed
- [ ] Announce release (social media, etc.)

## Security Considerations

- Never commit API tokens to the repository
- Use GitHub secrets for sensitive data
- Regularly rotate API tokens
- Review dependencies for security vulnerabilities before release 