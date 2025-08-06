# Common Issues & Solutions

This guide covers the most common issues you might encounter when using Rhema and provides step-by-step solutions to resolve them.

## 🚨 Critical Issues

### Rhema Won't Start

#### Issue: "Command not found: rhema"
**Symptoms**: Terminal returns "command not found" when trying to run `rhema`

**Solutions**:
```bash
# Check if Rhema is installed
which rhema

# If not found, reinstall Rhema
cargo install --force rhema

# Add Cargo bin to PATH (if needed)
export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
```

#### Issue: "Permission denied"
**Symptoms**: Permission errors when running Rhema commands

**Solutions**:
```bash
# Fix executable permissions
chmod +x $(which rhema)

# Fix project directory permissions
chmod -R 755 .rhema/

# Check file ownership
ls -la .rhema/
```

### Configuration Problems

#### Issue: "Invalid configuration file"
**Symptoms**: Rhema fails to start due to configuration errors

**Solutions**:
```bash
# Validate configuration
rhema config validate

# Reset to default configuration
rhema config reset

# Check configuration syntax
rhema config check
```

#### Issue: "Missing required configuration"
**Symptoms**: Rhema reports missing required settings

**Solutions**:
```bash
# Set required configuration
rhema config set --global project.name "my-project"
rhema config set --global editor.default "code"

# Initialize default configuration
rhema init --force
```

## 🔧 Compilation Issues

### Rust Compilation Errors

#### Issue: "Failed to compile"
**Symptoms**: Cargo build fails with compilation errors

**Solutions**:
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build

# Check for dependency conflicts
cargo tree

# Update dependencies
cargo update
```

#### Issue: "Version conflicts"
**Symptoms**: Dependency version conflicts during compilation

**Solutions**:
```bash
# Check dependency tree
cargo tree --duplicates

# Update Cargo.lock
cargo update

# Force resolution
cargo build --locked
```

### Crate-Specific Issues

#### Issue: "Knowledge crate compilation errors"
**Symptoms**: Knowledge-related features fail to compile

**Solutions**:
```bash
# Check knowledge crate status
cargo check -p rhema-knowledge

# Update vector store dependencies
cargo update -p qdrant-client
cargo update -p reqwest

# Rebuild with specific features
cargo build --features "knowledge"
```

## 📁 File System Issues

### Lock File Problems

#### Issue: "Lock file corrupted"
**Symptoms**: Rhema reports lock file corruption

**Solutions**:
```bash
# Validate lock file
rhema lock validate

# Repair lock file
rhema lock repair

# Regenerate lock file
rhema lock regenerate

# Backup and recreate
cp .rhema/lock.yaml .rhema/lock.yaml.backup
rhema lock generate
```

#### Issue: "Lock file conflicts"
**Symptoms**: Conflicts between different lock files

**Solutions**:
```bash
# Resolve conflicts
rhema lock resolve

# Merge lock files
rhema lock merge

# Force resolution
rhema lock resolve --force
```

### Cache Issues

#### Issue: "Cache corruption"
**Symptoms**: Cached data appears corrupted or invalid

**Solutions**:
```bash
# Clear cache
rhema cache clear

# Validate cache
rhema cache validate

# Rebuild cache
rhema cache rebuild
```

#### Issue: "Cache performance problems"
**Symptoms**: Slow performance due to cache issues

**Solutions**:
```bash
# Optimize cache
rhema cache optimize

# Check cache size
rhema cache info

# Clean old cache entries
rhema cache cleanup
```

## 🔗 Integration Issues

### Git Integration Problems

#### Issue: "Git hooks not working"
**Symptoms**: Git hooks fail to execute Rhema commands

**Solutions**:
```bash
# Reinstall Git hooks
rhema git setup-hooks --force

# Check hook permissions
ls -la .git/hooks/

# Test hook execution
.git/hooks/pre-commit
```

#### Issue: "Git repository not found"
**Symptoms**: Rhema can't find Git repository

**Solutions**:
```bash
# Initialize Git repository
git init

# Check Git status
git status

# Set up Git integration
rhema git setup
```

### Editor Integration Issues

#### Issue: "VS Code extension not working"
**Symptoms**: VS Code extension fails to load or function

**Solutions**:
```bash
# Check extension installation
code --list-extensions | grep rhema

# Reinstall extension
code --uninstall-extension rhema
code --install-extension rhema

# Check extension logs
code --verbose
```

#### Issue: "Editor commands not available"
**Symptoms**: Rhema commands not available in editor

**Solutions**:
```bash
# Check Rhema installation
which rhema

# Verify PATH in editor
echo $PATH

# Restart editor
code --restart
```

## 🚀 Performance Issues

### Slow Startup

#### Issue: "Rhema takes too long to start"
**Symptoms**: Rhema startup is slow

**Solutions**:
```bash
# Check startup time
time rhema --version

# Optimize configuration
rhema config optimize

# Disable unnecessary features
rhema config set --global performance.lazy_loading true
```

### Memory Issues

#### Issue: "High memory usage"
**Symptoms**: Rhema uses excessive memory

**Solutions**:
```bash
# Check memory usage
rhema performance memory

# Optimize memory settings
rhema config set --global performance.cache_size "50MB"
rhema config set --global performance.max_memory "100MB"

# Clear memory cache
rhema cache clear --memory
```

### Network Issues

#### Issue: "Network timeouts"
**Symptoms**: Network operations timeout

**Solutions**:
```bash
# Check network connectivity
rhema network test

# Increase timeout settings
rhema config set --global network.timeout "30s"

# Use offline mode
rhema --offline
```

## 🔍 Data Issues

### Todo Management Problems

#### Issue: "Todos not saving"
**Symptoms**: Todo changes are not persisted

**Solutions**:
```bash
# Check file permissions
ls -la .rhema/todos.yaml

# Force save
rhema todo save --force

# Validate todo file
rhema todo validate
```

#### Issue: "Todo search not working"
**Symptoms**: Todo search returns incorrect results

**Solutions**:
```bash
# Rebuild search index
rhema todo index --rebuild

# Clear search cache
rhema cache clear --search

# Validate search configuration
rhema config validate --search
```

### Insight Management Problems

#### Issue: "Insights not loading"
**Symptoms**: Insights fail to load or display

**Solutions**:
```bash
# Check insight file
rhema insight validate

# Rebuild insight index
rhema insight index --rebuild

# Import insights from backup
rhema insight import --file insights-backup.yaml
```

## 🛠️ Advanced Troubleshooting

### Debug Mode

Enable debug mode for detailed error information:
```bash
# Enable debug logging
export RHEMA_LOG_LEVEL=debug
rhema --debug

# Check debug logs
rhema logs --level debug
```

### System Diagnostics

Run comprehensive system diagnostics:
```bash
# Full system check
rhema diagnose

# Check system requirements
rhema system check

# Generate diagnostic report
rhema diagnose --report diagnostic-report.txt
```

### Recovery Procedures

#### Complete Reset
If all else fails, perform a complete reset:
```bash
# Backup current configuration
cp -r .rhema .rhema.backup

# Remove Rhema configuration
rm -rf .rhema

# Reinitialize Rhema
rhema init --force

# Restore from backup (if needed)
cp .rhema.backup/todos.yaml .rhema/
cp .rhema.backup/insights.yaml .rhema/
```

## 📞 Getting Help

### Self-Service Resources
- **Documentation**: Check the [main documentation](../index.md)
- **Examples**: Review [example projects](../../examples/README.md)
- **Configuration**: See [configuration reference](../reference/global-config-reference.md)

### Community Support
- **GitHub Issues**: Report bugs at [GitHub Issues](https://github.com/fugue-ai/rhema/issues)
- **Discussions**: Ask questions at [GitHub Discussions](https://github.com/fugue-ai/rhema/discussions)
- **Documentation**: Contribute to documentation improvements

### Professional Support
- **Enterprise Support**: Contact for enterprise support options
- **Consulting**: Get help with custom implementations
- **Training**: Schedule training sessions for your team

## 🔄 Prevention Tips

### Best Practices
1. **Regular Backups**: Backup your `.rhema` directory regularly
2. **Version Control**: Keep Rhema files in version control
3. **Configuration Management**: Use consistent configuration across environments
4. **Monitoring**: Monitor Rhema performance and logs
5. **Updates**: Keep Rhema and dependencies updated

### Maintenance Schedule
- **Daily**: Check Rhema status and logs
- **Weekly**: Validate configuration and lock files
- **Monthly**: Update Rhema and dependencies
- **Quarterly**: Review and optimize configuration

---

**Need More Help?**
If you can't find a solution here, please:
1. Check the [debug logs](#debug-mode) for detailed error information
2. Search existing [GitHub Issues](https://github.com/fugue-ai/rhema/issues)
3. Create a new issue with detailed information about your problem
4. Include system information, error messages, and steps to reproduce 