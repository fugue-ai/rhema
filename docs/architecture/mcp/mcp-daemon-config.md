# Rhema MCP Daemon Configuration Guide


## Overview


The Rhema MCP Daemon can be configured through a YAML configuration file or command-line arguments. This guide covers all configuration options and provides examples for different deployment scenarios.

## Configuration File Format


The daemon uses YAML configuration files. The default location is `rhema-mcp.yaml` in the current directory.

### Basic Configuration Structure


```yaml
# Rhema MCP Daemon Configuration


host: "127.0.0.1"
port: 8080
unix_socket: "/tmp/rhema-mcp.sock"
redis_url: "redis://localhost:6379"

auth:
  enabled: false
  api_key: "your-secret-api-key"
  jwt_secret: "your-jwt-secret-key"
  allowed_origins:

    - "*"

watcher:
  enabled: true
  watch_dirs:

    - ".rhema"
  file_patterns:

    - "*.yaml"

    - "*.yml"
  debounce_ms: 100

cache:
  memory_enabled: true
  redis_enabled: false
  ttl_seconds: 3600
  max_size: 10000

logging:
  level: "info"
  structured: true
  file: "/var/log/rhema-mcp.log"
```

## Configuration Options


### Network Configuration


#### `host` (string, default: "127.0.0.1")


The host address to bind the HTTP server to.

```yaml
host: "0.0.0.0"  # Bind to all interfaces
host: "192.168.1.100"  # Bind to specific IP
```

#### `port` (integer, default: 8080)


The port number for the HTTP server.

```yaml
port: 8080
port: 9000  # Custom port
```

#### `unix_socket` (string, optional)


Path to Unix domain socket for local communication.

```yaml
unix_socket: "/tmp/rhema-mcp.sock"
unix_socket: "/var/run/rhema-mcp.sock"
```

#### `redis_url` (string, optional)


Redis connection URL for distributed caching.

```yaml
redis_url: "redis://localhost:6379"
redis_url: "redis://user:password@redis.example.com:6379/0"
redis_url: "redis+sentinel://sentinel1:26379,sentinel2:26379/mymaster"
```

### Authentication Configuration


#### `auth.enabled` (boolean, default: false)


Enable authentication for API access.

```yaml
auth:
  enabled: true
```

#### `auth.api_key` (string, optional)


API key for simple token-based authentication.

```yaml
auth:
  enabled: true
  api_key: "your-secret-api-key-here"
```

#### `auth.jwt_secret` (string, optional)


Secret key for JWT token generation and validation.

```yaml
auth:
  enabled: true
  jwt_secret: "your-super-secret-jwt-key"
```

#### `auth.allowed_origins` (array, default: ["*"])


List of allowed CORS origins.

```yaml
auth:
  allowed_origins:

    - "https://app.example.com"

    - "http://localhost:3000"

    - "*"  # Allow all origins (development only)
```

### File System Watcher Configuration


#### `watcher.enabled` (boolean, default: true)


Enable file system watching for real-time updates.

```yaml
watcher:
  enabled: true
```

#### `watcher.watch_dirs` (array, default: [".rhema"])


Directories to watch for changes.

```yaml
watcher:
  watch_dirs:

    - ".rhema"

    - "config"

    - "docs"
```

#### `watcher.file_patterns` (array, default: ["*.yaml", "*.yml"])


File patterns to watch for changes.

```yaml
watcher:
  file_patterns:

    - "*.yaml"

    - "*.yml"

    - "*.json"

    - "*.toml"
```

#### `watcher.debounce_ms` (integer, default: 100)


Debounce interval in milliseconds to prevent excessive events.

```yaml
watcher:
  debounce_ms: 100  # 100ms debounce
  debounce_ms: 500  # 500ms debounce for slower systems
```

### Cache Configuration


#### `cache.memory_enabled` (boolean, default: true)


Enable in-memory caching.

```yaml
cache:
  memory_enabled: true
```

#### `cache.redis_enabled` (boolean, default: false)


Enable Redis caching for distributed environments.

```yaml
cache:
  redis_enabled: true
  redis_url: "redis://localhost:6379"
```

#### `cache.ttl_seconds` (integer, default: 3600)


Cache TTL (Time To Live) in seconds.

```yaml
cache:
  ttl_seconds: 3600  # 1 hour
  ttl_seconds: 86400  # 24 hours
```

#### `cache.max_size` (integer, default: 10000)


Maximum number of items in memory cache.

```yaml
cache:
  max_size: 10000
  max_size: 50000  # Larger cache for high-traffic systems
```

### Logging Configuration


#### `logging.level` (string, default: "info")


Log level (trace, debug, info, warn, error).

```yaml
logging:
  level: "info"
  level: "debug"  # More verbose logging
  level: "warn"   # Less verbose logging
```

#### `logging.structured` (boolean, default: true)


Enable structured JSON logging.

```yaml
logging:
  structured: true  # JSON format
  structured: false # Plain text format
```

#### `logging.file` (string, optional)


Log file path for file-based logging.

```yaml
logging:
  file: "/var/log/rhema-mcp.log"
  file: "logs/rhema-mcp.log"
```

## Configuration Examples


### Development Configuration


```yaml
# Development configuration


host: "127.0.0.1"
port: 8080
unix_socket: "/tmp/rhema-mcp-dev.sock"

auth:
  enabled: false  # No authentication for development

watcher:
  enabled: true
  watch_dirs:

    - ".rhema"
  file_patterns:

    - "*.yaml"

    - "*.yml"
  debounce_ms: 100

cache:
  memory_enabled: true
  redis_enabled: false
  ttl_seconds: 1800  # 30 minutes
  max_size: 1000

logging:
  level: "debug"
  structured: false
  file: "logs/rhema-mcp-dev.log"
```

### Production Configuration


```yaml
# Production configuration


host: "0.0.0.0"
port: 8080
unix_socket: "/var/run/rhema-mcp.sock"
redis_url: "redis://redis.example.com:6379"

auth:
  enabled: true
  api_key: "${Rhema_API_KEY}"  # Use environment variable
  jwt_secret: "${Rhema_JWT_SECRET}"
  allowed_origins:

    - "https://app.example.com"

    - "https://api.example.com"

watcher:
  enabled: true
  watch_dirs:

    - ".rhema"

    - "config"
  file_patterns:

    - "*.yaml"

    - "*.yml"
  debounce_ms: 200

cache:
  memory_enabled: true
  redis_enabled: true
  ttl_seconds: 3600
  max_size: 50000

logging:
  level: "info"
  structured: true
  file: "/var/log/rhema-mcp.log"
```

### Docker Configuration


```yaml
# Docker configuration


host: "0.0.0.0"
port: 8080
unix_socket: "/tmp/rhema-mcp.sock"
redis_url: "redis://redis:6379"

auth:
  enabled: true
  api_key: "${Rhema_API_KEY}"
  jwt_secret: "${Rhema_JWT_SECRET}"
  allowed_origins:

    - "*"  # Configure based on your setup

watcher:
  enabled: true
  watch_dirs:

    - ".rhema"
  file_patterns:

    - "*.yaml"

    - "*.yml"
  debounce_ms: 100

cache:
  memory_enabled: true
  redis_enabled: true
  ttl_seconds: 3600
  max_size: 10000

logging:
  level: "info"
  structured: true
  # No file logging in Docker (use stdout)


```

### Kubernetes Configuration


```yaml
# Kubernetes configuration


host: "0.0.0.0"
port: 8080
# No Unix socket in Kubernetes


redis_url: "redis://rhema-redis:6379"

auth:
  enabled: true
  api_key: "${Rhema_API_KEY}"
  jwt_secret: "${Rhema_JWT_SECRET}"
  allowed_origins:

    - "https://your-app.example.com"

watcher:
  enabled: true
  watch_dirs:

    - ".rhema"
  file_patterns:

    - "*.yaml"

    - "*.yml"
  debounce_ms: 100

cache:
  memory_enabled: true
  redis_enabled: true
  ttl_seconds: 3600
  max_size: 10000

logging:
  level: "info"
  structured: true
  # Use stdout for Kubernetes logging


```

### High-Performance Configuration


```yaml
# High-performance configuration


host: "0.0.0.0"
port: 8080
unix_socket: "/var/run/rhema-mcp.sock"
redis_url: "redis://redis-cluster:6379"

auth:
  enabled: true
  api_key: "${Rhema_API_KEY}"
  jwt_secret: "${Rhema_JWT_SECRET}"
  allowed_origins:

    - "https://app.example.com"

watcher:
  enabled: true
  watch_dirs:

    - ".rhema"
  file_patterns:

    - "*.yaml"

    - "*.yml"
  debounce_ms: 50  # Faster response

cache:
  memory_enabled: true
  redis_enabled: true
  ttl_seconds: 7200  # 2 hours
  max_size: 100000  # Large cache

logging:
  level: "warn"  # Less verbose for performance
  structured: true
  file: "/var/log/rhema-mcp.log"
```

## Environment Variables


The daemon supports environment variables for sensitive configuration:

```bash
# Authentication


export Rhema_API_KEY="your-secret-api-key"
export Rhema_JWT_SECRET="your-jwt-secret"

# Redis


export Rhema_REDIS_URL="redis://localhost:6379"

# Logging


export Rhema_LOG_LEVEL="info"
export Rhema_LOG_FILE="/var/log/rhema-mcp.log"
```

## Command-Line Configuration


You can override configuration file settings with command-line arguments:

```bash
# Start with custom host and port


rhema daemon start --host 0.0.0.0 --port 9000

# Start with authentication


rhema daemon start --auth --api-key "your-key"

# Start with custom config file


rhema daemon start --config /path/to/config.yaml

# Start with Unix socket only


rhema daemon start --unix-socket /tmp/rhema.sock

# Start with Redis


rhema daemon start --redis-url "redis://localhost:6379"
```

## Configuration Validation


The daemon validates configuration on startup:

```bash
# Validate configuration without starting


rhema daemon config --validate

# Generate default configuration


rhema daemon config --generate

# Generate configuration with comments


rhema daemon config --generate --comments
```

## Security Considerations


### Production Security


1. **Authentication**: Always enable authentication in production

2. **API Keys**: Use strong, randomly generated API keys

3. **JWT Secrets**: Use cryptographically secure JWT secrets

4. **CORS**: Restrict allowed origins to your application domains

5. **Network**: Bind to specific interfaces, not 0.0.0.0 unless necessary

6. **File Permissions**: Secure Unix socket file permissions

### Example Secure Configuration


```yaml
host: "127.0.0.1"  # Bind to localhost only
port: 8080
unix_socket: "/var/run/rhema-mcp.sock"

auth:
  enabled: true
  api_key: "${Rhema_API_KEY}"
  jwt_secret: "${Rhema_JWT_SECRET}"
  allowed_origins:

    - "https://your-app.example.com"

    - "https://api.your-app.example.com"

# ... rest of configuration


```

## Monitoring and Observability


### Health Checks


The daemon provides health check endpoints:

```bash
# Health check


curl http://localhost:8080/health

# Detailed health information


curl http://localhost:8080/health | jq
```

### Metrics


Enable metrics collection:

```yaml
logging:
  level: "info"
  structured: true
  metrics: true  # Enable metrics collection
```

### Logging Best Practices


1. **Structured Logging**: Use structured logging in production

2. **Log Levels**: Use appropriate log levels

3. **Log Rotation**: Implement log rotation for file logging

4. **Centralized Logging**: Send logs to centralized logging system

## Troubleshooting


### Common Issues


1. **Port Already in Use**: Change port or stop conflicting service

2. **Permission Denied**: Check file permissions for Unix socket

3. **Redis Connection Failed**: Verify Redis URL and connectivity

4. **Authentication Errors**: Check API key and JWT secret configuration

### Debug Mode


Enable debug logging for troubleshooting:

```yaml
logging:
  level: "debug"
  structured: false  # Plain text for easier reading
```

### Configuration Validation


Validate your configuration:

```bash
rhema daemon config --validate --config your-config.yaml
``` 