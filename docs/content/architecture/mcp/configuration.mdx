# MCP Configuration Guide

This document provides comprehensive configuration options for Rhema's Model Context Protocol (MCP) implementation, including daemon settings, protocol configuration, and deployment options.

## Configuration Overview

Rhema's MCP implementation can be configured through multiple methods:

- **Configuration Files**: TOML or YAML configuration files
- **Environment Variables**: Runtime configuration overrides
- **Command Line Arguments**: Direct parameter specification
- **Default Values**: Sensible defaults for common use cases

## Configuration File Format

### TOML Configuration

The primary configuration format is TOML, which provides a clean and readable structure:

```toml
# Rhema MCP Daemon Configuration
[daemon]
host = "127.0.0.1"
port = 8080
log_level = "info"
workers = 4

[cache]
type = "redis"
url = "redis://localhost:6379"
ttl = 3600
max_size = 10000

[security]
auth_required = true
api_key = "your-secret-api-key"
allowed_origins = ["http://localhost:3000", "https://yourdomain.com"]

[mcp]
protocol_version = "2025-06-18"
enable_websocket = true
enable_http = true
enable_unix_socket = true
```

### YAML Configuration

Alternative YAML format for those who prefer it:

```yaml
# Rhema MCP Daemon Configuration
daemon:
  host: "127.0.0.1"
  port: 8080
  log_level: "info"
  workers: 4

cache:
  type: "redis"
  url: "redis://localhost:6379"
  ttl: 3600
  max_size: 10000

security:
  auth_required: true
  api_key: "your-secret-api-key"
  allowed_origins:
    - "http://localhost:3000"
    - "https://yourdomain.com"

mcp:
  protocol_version: "2025-06-18"
  enable_websocket: true
  enable_http: true
  enable_unix_socket: true
```

## Configuration Sections

### Daemon Configuration

Controls the main daemon behavior and networking:

```toml
[daemon]
# Network binding
host = "127.0.0.1"                    # Bind address
port = 8080                           # Port number
unix_socket = "/tmp/rhema-mcp.sock"   # Unix socket path (optional)

# Performance settings
workers = 4                           # Number of worker threads
max_connections = 1000                # Maximum concurrent connections
connection_timeout = 30               # Connection timeout in seconds

# Logging configuration
log_level = "info"                    # debug, info, warn, error
log_format = "json"                   # json, text
log_file = "/var/log/rhema-mcp.log"   # Log file path (optional)

# Health monitoring
health_check_interval = 30            # Health check interval in seconds
metrics_enabled = true                # Enable metrics collection
```

### Cache Configuration

Configures caching behavior for performance optimization:

```toml
[cache]
# Cache type selection
type = "redis"                        # redis, memory, hybrid

# Redis configuration (when type = "redis")
url = "redis://localhost:6379"        # Redis connection URL
password = "redis-password"           # Redis password (optional)
database = 0                          # Redis database number
pool_size = 10                        # Connection pool size

# Memory cache configuration (when type = "memory")
max_size = 10000                      # Maximum cache entries
ttl = 3600                            # Time-to-live in seconds
eviction_policy = "lru"               # lru, lfu, fifo

# Hybrid cache configuration (when type = "hybrid")
memory_size = 1000                    # Memory cache size
redis_ttl = 7200                      # Redis TTL in seconds
```

### Security Configuration

Controls authentication and authorization:

```toml
[security]
# Authentication settings
auth_required = true                  # Require authentication
api_key = "your-secret-api-key"       # API key for authentication
jwt_secret = "your-jwt-secret"        # JWT secret for token auth
jwt_expiry = 3600                     # JWT token expiry in seconds

# CORS configuration
allowed_origins = [                   # Allowed CORS origins
    "http://localhost:3000",
    "https://yourdomain.com"
]
allowed_methods = [                   # Allowed HTTP methods
    "GET",
    "POST",
    "PUT",
    "DELETE"
]
allowed_headers = [                   # Allowed HTTP headers
    "Content-Type",
    "Authorization"
]

# Rate limiting
rate_limit_enabled = true             # Enable rate limiting
rate_limit_requests = 100             # Requests per minute
rate_limit_window = 60                # Time window in seconds
```

### MCP Protocol Configuration

Configures the Model Context Protocol behavior:

```toml
[mcp]
# Protocol version
protocol_version = "2025-06-18"       # MCP protocol version

# Transport configuration
enable_websocket = true               # Enable WebSocket transport
enable_http = true                    # Enable HTTP transport
enable_unix_socket = true             # Enable Unix socket transport

# WebSocket settings
websocket_path = "/ws"                # WebSocket endpoint path
websocket_max_message_size = 1048576  # Max message size in bytes

# HTTP settings
http_read_timeout = 30                # HTTP read timeout in seconds
http_write_timeout = 30               # HTTP write timeout in seconds
http_max_body_size = 1048576          # Max request body size in bytes

# Unix socket settings
unix_socket_permissions = 0o600        # Unix socket file permissions
```

### File System Watching

Configures real-time file system monitoring:

```toml
[watcher]
# File watching settings
enabled = true                        # Enable file system watching
watch_dirs = [".rhema"]               # Directories to watch
file_patterns = ["*.yaml", "*.yml"]   # File patterns to watch
ignore_patterns = ["*.tmp", "*.bak"]  # Patterns to ignore

# Performance settings
debounce_ms = 100                     # Debounce time in milliseconds
batch_size = 100                      # Batch size for file events
max_events = 1000                     # Maximum events per batch
```

## Environment Variables

Configuration can be overridden using environment variables:

```bash
# Daemon configuration
export RHEMA_MCP_HOST="0.0.0.0"
export RHEMA_MCP_PORT="8080"
export RHEMA_MCP_LOG_LEVEL="debug"

# Cache configuration
export RHEMA_MCP_CACHE_TYPE="redis"
export RHEMA_MCP_CACHE_URL="redis://localhost:6379"

# Security configuration
export RHEMA_MCP_AUTH_REQUIRED="true"
export RHEMA_MCP_API_KEY="your-secret-key"

# MCP protocol configuration
export RHEMA_MCP_PROTOCOL_VERSION="2025-06-18"
export RHEMA_MCP_ENABLE_WEBSOCKET="true"
```

## Command Line Configuration

Configuration can be specified directly via command line arguments:

```bash
# Basic daemon start
rhema daemon start

# Start with specific configuration
rhema daemon start --config /path/to/config.toml

# Override configuration values
rhema daemon start \
  --host 0.0.0.0 \
  --port 8080 \
  --log-level debug \
  --auth-required \
  --api-key "your-secret-key"

# Development mode
rhema daemon start --dev --foreground
```

## Configuration Validation

Rhema validates configuration before starting the daemon:

```bash
# Validate configuration file
rhema daemon config validate --file /path/to/config.toml

# Generate default configuration
rhema daemon config generate --output config.toml

# Show current configuration
rhema daemon config show

# Test configuration
rhema daemon config test --file /path/to/config.toml
```

## Deployment Configurations

### Development Configuration

```toml
[daemon]
host = "127.0.0.1"
port = 8080
log_level = "debug"
workers = 2

[cache]
type = "memory"
max_size = 1000
ttl = 1800

[security]
auth_required = false

[mcp]
enable_websocket = true
enable_http = true
enable_unix_socket = true
```

### Production Configuration

```toml
[daemon]
host = "0.0.0.0"
port = 8080
log_level = "info"
workers = 8
max_connections = 1000

[cache]
type = "redis"
url = "redis://redis-cluster:6379"
pool_size = 20
ttl = 7200

[security]
auth_required = true
api_key = "${RHEMA_MCP_API_KEY}"
allowed_origins = ["https://yourdomain.com"]

[mcp]
protocol_version = "2025-06-18"
enable_websocket = true
enable_http = true
```

### Docker Configuration

```toml
[daemon]
host = "0.0.0.0"
port = 8080
log_level = "info"

[cache]
type = "redis"
url = "redis://redis:6379"

[security]
auth_required = true
api_key = "${RHEMA_MCP_API_KEY}"

[mcp]
enable_websocket = true
enable_http = true
```

## Configuration Best Practices

### Security

1. **Use Environment Variables**: Store sensitive values in environment variables
2. **Enable Authentication**: Always enable authentication in production
3. **Restrict CORS**: Limit allowed origins to necessary domains
4. **Use HTTPS**: Enable TLS/SSL in production environments
5. **Rotate API Keys**: Regularly rotate API keys and secrets

### Performance

1. **Optimize Cache Settings**: Configure cache size and TTL based on usage patterns
2. **Adjust Worker Count**: Set worker count based on CPU cores and load
3. **Monitor Connections**: Set appropriate connection limits
4. **Use Redis**: Use Redis for distributed caching in multi-instance deployments

### Monitoring

1. **Enable Metrics**: Enable metrics collection for monitoring
2. **Configure Logging**: Set appropriate log levels and formats
3. **Health Checks**: Configure health check intervals
4. **File Watching**: Optimize file watching for your use case

## Troubleshooting

### Common Configuration Issues

1. **Port Already in Use**
   ```bash
   # Check port usage
   netstat -tulpn | grep :8080
   
   # Change port in configuration
   port = 8081
   ```

2. **Redis Connection Issues**
   ```bash
   # Test Redis connection
   redis-cli ping
   
   # Check Redis configuration
   rhema daemon config test --cache-url "redis://localhost:6379"
   ```

3. **Permission Issues**
   ```bash
   # Check file permissions
   ls -la /path/to/config.toml
   
   # Fix permissions
   chmod 600 /path/to/config.toml
   ```

### Debug Configuration

```bash
# Enable debug logging
rhema daemon start --log-level debug

# Validate configuration
rhema daemon config validate --verbose

# Show effective configuration
rhema daemon config show --include-defaults
``` 