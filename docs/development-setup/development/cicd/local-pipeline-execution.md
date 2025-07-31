# Local Pipeline Execution with Act

This guide explains how to run the pull request pipeline locally using [nektos/act](https://github.com/nektos/act), which allows you to test GitHub Actions workflows locally without pushing to the repository.

## Prerequisites

### 1. Install Act

**macOS (using Homebrew):**
```bash
brew install act
```

**Linux:**
```bash
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
```

**Windows:**
```bash
choco install act-cli
```

### 2. Install Docker

Act requires Docker to run. Install Docker Desktop or Docker Engine:

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Docker Engine](https://docs.docker.com/engine/install/)

### 3. Setup Environment

1. Copy the example environment file:
```bash
cp env.local.example .env.local
```

2. Customize `.env.local` as needed:
```bash
# Edit the file to set your preferences
nano .env.local
```

## Running the Pipeline Locally

### Basic Usage

Run the entire pull request pipeline:
```bash
act pull_request
```

Run a specific job:
```bash
act pull_request -j test
```

### Manual Trigger

Simulate a manual workflow dispatch:
```bash
act workflow_dispatch
```

With custom inputs:
```bash
act workflow_dispatch \
  -e <(echo '{"inputs": {"run_tests": "true", "run_security": "false", "run_validation": "true"}}')
```

### Advanced Options

**Dry run (list what would be executed):**
```bash
act pull_request --dryrun
```

**Verbose output:**
```bash
act pull_request -v
```

**Use specific image:**
```bash
act pull_request -P ubuntu-latest=catthehacker/ubuntu:act-latest
```

**Run with secrets:**
```bash
act pull_request --secret-file .secrets
```

## Configuration Files

### .actrc

The `.actrc` file contains default configuration for act:

```bash
# Use medium-sized image for better compatibility
-P ubuntu-latest=catthehacker/ubuntu:act-latest

# Set environment variables for local execution
--env-file .env.local

# Enable verbose output for debugging
-v

# Use bind mounts for better performance
--bind

# Set working directory
--workflows .github/workflows/

# Enable reuse of containers
--reuse
```

### .env.local

Environment variables for local execution:

```bash
# Local development settings
ACT_LOCAL=true

# Skip external services for local testing
SKIP_CODECOV=true
SKIP_SECURITY_SCAN=true

# Rust configuration
RUST_VERSION=stable
RHEMA_RUN_INTEGRATION_TESTS=1

# Cargo configuration
CARGO_TERM_COLOR=always
RUST_BACKTRACE=1
RUST_LOG=info
```

## Workflow Modifications for Local Execution

The pull request workflow has been enhanced with local execution support:

### Conditional Steps

- **System Dependencies**: Skipped in local mode
- **Security Scanning**: Can be disabled via `SKIP_SECURITY_SCAN`
- **Codecov Upload**: Can be disabled via `SKIP_CODECOV`

### Environment Variables

- `ACT_LOCAL`: Indicates local execution mode
- `SKIP_CODECOV`: Skip Codecov integration
- `SKIP_SECURITY_SCAN`: Skip security scanning steps

## Troubleshooting

### Common Issues

**1. Docker Permission Issues**
```bash
# Add user to docker group
sudo usermod -aG docker $USER
# Log out and back in, or restart Docker
```

**2. Image Pull Failures**
```bash
# Pull the image manually
docker pull catthehacker/ubuntu:act-latest
```

**3. Memory Issues**
```bash
# Increase Docker memory limit in Docker Desktop settings
# Or use a smaller image
act pull_request -P ubuntu-latest=node:16-bullseye
```

**4. Network Issues**
```bash
# Use host networking
act pull_request --network host
```

### Performance Optimization

**1. Use Bind Mounts**
```bash
act pull_request --bind
```

**2. Reuse Containers**
```bash
act pull_request --reuse
```

**3. Use Smaller Images**
```bash
# For faster startup
act pull_request -P ubuntu-latest=node:16-bullseye
```

### Debugging

**Enable Verbose Output:**
```bash
act pull_request -v
```

**Check Container Logs:**
```bash
docker logs <container_id>
```

**Run Single Step:**
```bash
act pull_request --list
act pull_request -s <step_name>
```

## Best Practices

### 1. Local Development Workflow

1. **Make changes** to your code
2. **Run tests locally** with act
3. **Fix issues** before pushing
4. **Push to GitHub** for full CI/CD

### 2. Environment Management

- Keep `.env.local` in `.gitignore`
- Use different environment files for different scenarios
- Document required environment variables

### 3. Performance

- Use bind mounts for faster file access
- Reuse containers when possible
- Skip external services in local mode

### 4. Security

- Never commit secrets to version control
- Use `.secrets` file for local testing
- Be cautious with security scanning in local mode

## Integration with Development Workflow

### Pre-commit Testing

Add to your development workflow:

```bash
# Before committing
act pull_request --dryrun
act pull_request -j test

# If tests pass, commit
git add .
git commit -m "Your commit message"
```

### Continuous Local Testing

Set up a watch script:

```bash
# Watch for changes and run tests
fswatch -o . | xargs -n1 -I{} act pull_request -j test
```

### IDE Integration

Configure your IDE to run act:

**VS Code (tasks.json):**
```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "Run Pipeline",
      "type": "shell",
      "command": "act",
      "args": ["pull_request"],
      "group": "test"
    }
  ]
}
```

## Comparison: Local vs Remote

| Feature | Local (Act) | Remote (GitHub) |
|---------|-------------|-----------------|
| Speed | Faster iteration | Full CI/CD |
| Resources | Limited by local machine | GitHub runners |
| External Services | Can be skipped | Full integration |
| Debugging | Easy access to logs | Limited access |
| Cost | Free | GitHub minutes |
| Dependencies | Local Docker setup | Managed by GitHub |

## Next Steps

1. **Install act** and Docker
2. **Copy environment file** and customize
3. **Run your first local pipeline**
4. **Integrate into your development workflow**
5. **Optimize for your specific needs**

For more information, see the [act documentation](https://github.com/nektos/act). 