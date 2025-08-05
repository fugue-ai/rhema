# Migration Guide: From docs.sh to nx Commands

## 🚀 Overview

The Rhema documentation management has been migrated from a custom `docs.sh` script to use `nx` commands integrated with `pnpm`. This provides better integration with the overall project build system and improved caching.

## 📋 Command Mapping

| Old Command | New Command | Description |
|-------------|-------------|-------------|
| `./docs.sh dev` | `pnpm run docs:dev` | Start development server |
| `./docs.sh build` | `pnpm run docs:build` | Build for production |
| `./docs.sh preview` | `pnpm run docs:preview` | Preview production build |
| `./docs.sh install` | `pnpm run docs:install` | Install dependencies |
| `./docs.sh clean` | `pnpm run docs:clean` | Clean build artifacts |
| `./docs.sh migrate` | `./migrate.sh` | Run migration script (unchanged) |
| `./docs.sh help` | `pnpm run docs:help` | Show help (see below) |

## 🆕 New Commands Available

The new `nx` integration provides additional commands not available in the old script:

| Command | Description |
|---------|-------------|
| `pnpm run docs:test` | Run tests |
| `pnpm run docs:check` | Check TypeScript |
| `pnpm run docs:lint` | Lint code |
| `pnpm run docs:format` | Format code |

## 🔧 How It Works

### Old System (docs.sh)
- Custom bash script with hardcoded commands
- Manual directory navigation
- No caching or optimization

### New System (nx + pnpm)
- Uses `nx` project configuration in `docs/project.json`
- Leverages `nx` caching for faster builds
- Integrated with the overall project build system
- Uses `pnpm` workspace filtering for dependency management

## 📁 Configuration Files

### docs/project.json
```json
{
  "name": "docs",
  "targets": {
    "build": {
      "executor": "nx:run-commands",
      "options": {
        "command": "pnpm build",
        "cwd": "docs"
      }
    },
    "dev": {
      "executor": "nx:run-commands", 
      "options": {
        "command": "pnpm dev",
        "cwd": "docs"
      }
    }
    // ... more targets
  }
}
```

### package.json (root)
```json
{
  "scripts": {
    "docs:dev": "npx nx dev --project=docs",
    "docs:build": "npx nx build --project=docs",
    "docs:preview": "npx nx preview --project=docs",
    // ... more scripts
  }
}
```

## 🚀 Benefits of the New System

### 1. **Better Integration**
- Seamless integration with the overall project build system
- Consistent with other project commands (rust:*, etc.)

### 2. **Improved Performance**
- `nx` caching for faster subsequent builds
- Parallel execution capabilities
- Optimized dependency management

### 3. **Enhanced Developer Experience**
- Better error reporting and debugging
- Consistent command structure across the project
- IDE integration and autocompletion

### 4. **Future-Proof**
- Built on industry-standard tools (`nx`, `pnpm`)
- Easier to maintain and extend
- Better community support

## 🔄 Migration Steps

### 1. **Update Your Workflow**
Replace any scripts or documentation that reference `docs.sh` with the new `nx` commands.

### 2. **Update CI/CD Pipelines**
If you have CI/CD pipelines that use `docs.sh`, update them to use the new commands:

```yaml
# Old
- run: ./docs.sh build

# New  
- run: pnpm run docs:build
```

### 3. **Update Documentation**
Update any documentation that references the old commands.

### 4. **Test the New Commands**
Verify that all commands work as expected:

```bash
# Test development server
pnpm run docs:dev

# Test production build
pnpm run docs:build

# Test preview
pnpm run docs:preview
```

## 🗑️ Removing the Old Script

Once you've migrated to the new commands, you can safely remove the `docs.sh` script:

```bash
rm docs.sh
```

## ❓ Getting Help

### Show Available Commands
```bash
# List all available docs commands
pnpm run --filter=kitdocs-docs

# Show nx project info
npx nx show project docs
```

### Debug Issues
```bash
# Run with verbose output
npx nx dev --project=docs --verbose

# Check project configuration
npx nx graph --focus=docs
```

## 🎉 Conclusion

The migration to `nx` commands provides a more robust, performant, and maintainable solution for managing the Rhema documentation. The new system integrates seamlessly with the overall project architecture while providing additional features and better performance.

For any issues or questions about the migration, please refer to the main project documentation or create an issue in the project repository. 