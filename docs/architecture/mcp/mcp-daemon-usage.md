# GACP MCP Daemon Usage Guide

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Deployment Options](#deployment-options)
4. [Integration Examples](#integration-examples)
5. [Monitoring and Maintenance](#monitoring-and-maintenance)
6. [Troubleshooting](#troubleshooting)
7. [Best Practices](#best-practices)

## Installation

### Prerequisites

- Rust 1.70+ (for building from source)
- Git repository with GACP context files
- Optional: Redis for distributed caching

### Building from Source

```bash
# Clone the repository
git clone https://github.com/fugue-ai/gacp.git
cd gacp

# Build the project
cargo build --release

# Install globally
cargo install --path .
```

### Using Pre-built Binaries

Download the latest release from the [GitHub releases page](https://github.com/fugue-ai/gacp/releases).

## Quick Start

### 1. Initialize GACP Context

First, ensure you have a GACP-enabled repository:

```bash
# Initialize GACP in your repository
gacp init --scope-type service --scope-name my-service

# This creates the basic structure:
# .gacp/
# ├── scopes/
# │   └── my-service/
# │       ├── scope.yaml
# │       ├── knowledge.yaml
# │       ├── todos.yaml
# │       ├── decisions.yaml
# │       └── patterns.yaml
```

### 2. Start the Daemon

```bash
# Start with default configuration
gacp daemon start

# Start with custom configuration
gacp daemon start --host 0.0.0.0 --port 8080 --auth --api-key "your-secret-key"
```

### 3. Verify the Daemon

```bash
# Check health
curl http://localhost:8080/health

# List scopes
curl http://localhost:8080/scopes

# Execute a query
curl -X POST http://localhost:8080/query \
  -H "Content-Type: application/json" \
  -d '{"query": "SELECT * FROM scopes"}'
```

## Deployment Options

### Local Development

```bash
# Simple local development setup
gacp daemon start --host 127.0.0.1 --port 8080

# With file watching for development
gacp daemon start --watch --watch-dirs ".gacp,config"
```

### Docker Deployment

Create a `Dockerfile`:

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/gacp /usr/local/bin/gacp
WORKDIR /app
EXPOSE 8080
CMD ["gacp", "daemon", "start", "--host", "0.0.0.0", "--port", "8080"]
```

Create `docker-compose.yml`:

```yaml
version: '3.8'
services:
  gacp-mcp:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - .:/app
      - /tmp/gacp-mcp.sock:/tmp/gacp-mcp.sock
    environment:
      - GACP_API_KEY=your-secret-key
      - GACP_REDIS_URL=redis://redis:6379
    depends_on:
      - redis

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"
```

Run with Docker Compose:

```bash
docker-compose up -d
```

### Kubernetes Deployment

Create `k8s-deployment.yaml`:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: gacp-mcp
  labels:
    app: gacp-mcp
spec:
  replicas: 3
  selector:
    matchLabels:
      app: gacp-mcp
  template:
    metadata:
      labels:
        app: gacp-mcp
    spec:
      containers:
      - name: gacp-mcp
        image: gacp-mcp:latest
        ports:
        - containerPort: 8080
        env:
        - name: GACP_API_KEY
          valueFrom:
            secretKeyRef:
              name: gacp-secrets
              key: api-key
        - name: GACP_REDIS_URL
          value: "redis://gacp-redis:6379"
        volumeMounts:
        - name: gacp-config
          mountPath: /app/.gacp
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: gacp-config
        configMap:
          name: gacp-config

---
apiVersion: v1
kind: Service
metadata:
  name: gacp-mcp-service
spec:
  selector:
    app: gacp-mcp
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
  type: LoadBalancer
```

Deploy to Kubernetes:

```bash
kubectl apply -f k8s-deployment.yaml
```

### Systemd Service

Create `/etc/systemd/system/gacp-mcp.service`:

```ini
[Unit]
Description=GACP MCP Daemon
After=network.target

[Service]
Type=simple
User=gacp
Group=gacp
WorkingDirectory=/opt/gacp
ExecStart=/usr/local/bin/gacp daemon start --config /etc/gacp/gacp-mcp.yaml
Restart=always
RestartSec=10
Environment=GACP_API_KEY=your-secret-key

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable gacp-mcp
sudo systemctl start gacp-mcp
sudo systemctl status gacp-mcp
```

## Integration Examples

### Python Integration

```python
import requests
import json
from typing import Dict, Any, Optional

class GacpClient:
    def __init__(self, base_url: str = "http://localhost:8080", api_key: Optional[str] = None):
        self.base_url = base_url
        self.session = requests.Session()
        if api_key:
            self.session.headers.update({'Authorization': f'Bearer {api_key}'})
        self.session.headers.update({'Content-Type': 'application/json'})

    def health(self) -> Dict[str, Any]:
        """Get daemon health status."""
        response = self.session.get(f'{self.base_url}/health')
        response.raise_for_status()
        return response.json()

    def list_scopes(self) -> list:
        """List all available scopes."""
        response = self.session.get(f'{self.base_url}/scopes')
        response.raise_for_status()
        return response.json()

    def get_scope(self, scope_id: str) -> Optional[Dict[str, Any]]:
        """Get details for a specific scope."""
        response = self.session.get(f'{self.base_url}/scopes/{scope_id}')
        if response.status_code == 404:
            return None
        response.raise_for_status()
        return response.json()

    def execute_query(self, query: str, parameters: Optional[Dict[str, Any]] = None) -> Dict[str, Any]:
        """Execute a CQL query."""
        data = {'query': query}
        if parameters:
            data['parameters'] = parameters
        response = self.session.post(f'{self.base_url}/query', json=data)
        response.raise_for_status()
        return response.json()

    def get_knowledge(self, scope_id: str) -> Optional[Dict[str, Any]]:
        """Get knowledge base for a scope."""
        response = self.session.get(f'{self.base_url}/scopes/{scope_id}/knowledge')
        if response.status_code == 404:
            return None
        response.raise_for_status()
        return response.json()

# Usage example
client = GacpClient('http://localhost:8080', 'your-api-key')

# Check health
health = client.health()
print(f"Daemon status: {health['status']}")

# List scopes
scopes = client.list_scopes()
for scope in scopes:
    print(f"Scope: {scope['path']}")

# Execute query
result = client.execute_query("SELECT * FROM scopes WHERE type = 'service'")
print(f"Found {len(result['results'])} service scopes")

# Get knowledge
knowledge = client.get_knowledge('my-service')
if knowledge:
    print(f"Knowledge base: {knowledge['title']}")
```

### JavaScript/Node.js Integration

```javascript
class GacpClient {
  constructor(baseUrl = 'http://localhost:8080', apiKey = null) {
    this.baseUrl = baseUrl;
    this.apiKey = apiKey;
  }

  async request(endpoint, options = {}) {
    const headers = {
      'Content-Type': 'application/json',
      ...options.headers
    };

    if (this.apiKey) {
      headers['Authorization'] = `Bearer ${this.apiKey}`;
    }

    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      headers
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    return response.json();
  }

  async health() {
    return this.request('/health');
  }

  async listScopes() {
    return this.request('/scopes');
  }

  async getScope(scopeId) {
    return this.request(`/scopes/${scopeId}`);
  }

  async executeQuery(query, parameters = {}) {
    return this.request('/query', {
      method: 'POST',
      body: JSON.stringify({ query, parameters })
    });
  }

  async getKnowledge(scopeId) {
    return this.request(`/scopes/${scopeId}/knowledge`);
  }

  async getTodos(scopeId) {
    return this.request(`/scopes/${scopeId}/todos`);
  }

  async getDecisions(scopeId) {
    return this.request(`/scopes/${scopeId}/decisions`);
  }
}

// Usage example
const client = new GacpClient('http://localhost:8080', 'your-api-key');

async function main() {
  try {
    // Check health
    const health = await client.health();
    console.log(`Daemon status: ${health.status}`);

    // List scopes
    const scopes = await client.listScopes();
    console.log('Available scopes:', scopes.map(s => s.path));

    // Execute query
    const result = await client.executeQuery(
      "SELECT * FROM scopes WHERE type = 'service'"
    );
    console.log(`Found ${result.results.length} service scopes`);

    // Get knowledge for a scope
    const knowledge = await client.getKnowledge('my-service');
    if (knowledge) {
      console.log(`Knowledge base: ${knowledge.title}`);
    }
  } catch (error) {
    console.error('Error:', error.message);
  }
}

main();
```

### WebSocket Integration

```javascript
class GacpWebSocketClient {
  constructor(url = 'ws://localhost:8081/ws', apiKey = null) {
    this.url = url;
    this.apiKey = apiKey;
    this.ws = null;
    this.messageId = 0;
    this.pendingRequests = new Map();
  }

  connect() {
    return new Promise((resolve, reject) => {
      this.ws = new WebSocket(this.url);
      
      this.ws.onopen = () => {
        console.log('WebSocket connected');
        resolve();
      };

      this.ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        this.handleMessage(message);
      };

      this.ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        reject(error);
      };

      this.ws.onclose = () => {
        console.log('WebSocket disconnected');
      };
    });
  }

  handleMessage(message) {
    if (message.id && this.pendingRequests.has(message.id)) {
      const { resolve, reject } = this.pendingRequests.get(message.id);
      this.pendingRequests.delete(message.id);
      
      if (message.error) {
        reject(new Error(message.error.message));
      } else {
        resolve(message.result);
      }
    } else if (message.method === 'resources/changed') {
      // Handle notifications
      this.onResourceChanged?.(message.params);
    }
  }

  async sendRequest(method, params = {}) {
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      throw new Error('WebSocket not connected');
    }

    const id = ++this.messageId;
    const request = {
      jsonrpc: '2.0',
      id,
      method,
      params
    };

    return new Promise((resolve, reject) => {
      this.pendingRequests.set(id, { resolve, reject });
      this.ws.send(JSON.stringify(request));
    });
  }

  async subscribe(uri) {
    return this.sendRequest('resources/subscribe', { uri });
  }

  async unsubscribe(subscriptionId) {
    return this.sendRequest('resources/unsubscribe', { subscription_id: subscriptionId });
  }

  async health() {
    return this.sendRequest('system/health');
  }

  async executeQuery(query, parameters = {}) {
    return this.sendRequest('query/execute', { query, parameters });
  }

  disconnect() {
    if (this.ws) {
      this.ws.close();
    }
  }
}

// Usage example
const wsClient = new GacpWebSocketClient('ws://localhost:8081/ws', 'your-api-key');

// Handle resource changes
wsClient.onResourceChanged = (params) => {
  console.log(`Resource changed: ${params.uri} (${params.event_type})`);
};

async function main() {
  try {
    await wsClient.connect();
    
    // Subscribe to resource changes
    const subscription = await wsClient.subscribe('gacp://scopes/my-service');
    console.log('Subscribed:', subscription.subscription_id);

    // Execute query
    const result = await wsClient.executeQuery("SELECT * FROM scopes");
    console.log('Query result:', result);

    // Keep connection alive
    setInterval(async () => {
      const health = await wsClient.health();
      console.log('Health:', health.status);
    }, 30000);

  } catch (error) {
    console.error('Error:', error.message);
  }
}

main();
```

### Unix Socket Integration

```bash
#!/bin/bash

# Unix socket client example
SOCKET_PATH="/tmp/gacp-mcp.sock"

# Function to send JSON-RPC request
send_request() {
    local method="$1"
    local params="$2"
    local id="$3"
    
    local request=$(cat <<EOF
{"jsonrpc": "2.0", "id": $id, "method": "$method", "params": $params}
EOF
)
    
    echo "$request" | nc -U "$SOCKET_PATH"
}

# Health check
echo "Checking health..."
send_request "system/health" "{}" 1

# List scopes
echo "Listing scopes..."
send_request "resources/list" '{"uri": "gacp://scopes"}' 2

# Execute query
echo "Executing query..."
send_request "query/execute" '{"query": "SELECT * FROM scopes"}' 3
```

## Monitoring and Maintenance

### Health Monitoring

```bash
# Basic health check
curl -f http://localhost:8080/health || exit 1

# Detailed health information
curl http://localhost:8080/health | jq '.'

# Check specific metrics
curl http://localhost:8080/health | jq '.memory_usage'
curl http://localhost:8080/health | jq '.cache_hit_rate'
```

### Log Monitoring

```bash
# Follow logs
tail -f /var/log/gacp-mcp.log

# Search for errors
grep ERROR /var/log/gacp-mcp.log

# Monitor connection count
watch -n 5 'curl -s http://localhost:8080/health | jq ".connections"'
```

### Performance Monitoring

```bash
# Check memory usage
curl http://localhost:8080/health | jq '.memory_usage'

# Monitor cache performance
curl http://localhost:8080/stats | jq '.cache_stats'

# Check query performance
curl http://localhost:8080/stats | jq '.query_stats'
```

### Automated Monitoring Script

```bash
#!/bin/bash

# Monitoring script for GACP MCP Daemon
DAEMON_URL="http://localhost:8080"
ALERT_EMAIL="admin@example.com"

# Check if daemon is running
check_health() {
    local response=$(curl -s -w "%{http_code}" "$DAEMON_URL/health" -o /tmp/health.json)
    local status_code="${response: -3}"
    
    if [ "$status_code" != "200" ]; then
        echo "ERROR: Daemon health check failed with status $status_code"
        return 1
    fi
    
    local status=$(jq -r '.status' /tmp/health.json)
    if [ "$status" != "healthy" ]; then
        echo "ERROR: Daemon status is $status"
        return 1
    fi
    
    echo "OK: Daemon is healthy"
    return 0
}

# Check memory usage
check_memory() {
    local used=$(jq -r '.memory_usage.used' /tmp/health.json)
    local total=$(jq -r '.memory_usage.total' /tmp/health.json)
    local percentage=$((used * 100 / total))
    
    if [ "$percentage" -gt 80 ]; then
        echo "WARNING: High memory usage: ${percentage}%"
        return 1
    fi
    
    echo "OK: Memory usage: ${percentage}%"
    return 0
}

# Check cache performance
check_cache() {
    local hit_rate=$(jq -r '.cache_hit_rate' /tmp/health.json)
    local percentage=$(echo "$hit_rate * 100" | bc -l | cut -d. -f1)
    
    if [ "$percentage" -lt 80 ]; then
        echo "WARNING: Low cache hit rate: ${percentage}%"
        return 1
    fi
    
    echo "OK: Cache hit rate: ${percentage}%"
    return 0
}

# Main monitoring function
main() {
    local errors=0
    
    if ! check_health; then
        ((errors++))
    fi
    
    if ! check_memory; then
        ((errors++))
    fi
    
    if ! check_cache; then
        ((errors++))
    fi
    
    if [ "$errors" -gt 0 ]; then
        echo "ALERT: $errors issues detected with GACP MCP Daemon"
        # Send alert email
        echo "GACP MCP Daemon monitoring alert" | mail -s "Daemon Alert" "$ALERT_EMAIL"
        exit 1
    fi
    
    echo "All checks passed"
}

main
```

## Troubleshooting

### Common Issues

#### Daemon Won't Start

```bash
# Check if port is already in use
sudo lsof -i :8080

# Check if Unix socket file exists
ls -la /tmp/gacp-mcp.sock

# Check permissions
sudo chown gacp:gacp /tmp/gacp-mcp.sock
sudo chmod 660 /tmp/gacp-mcp.sock
```

#### Authentication Errors

```bash
# Check API key configuration
grep api_key /etc/gacp/gacp-mcp.yaml

# Test with curl
curl -H "Authorization: Bearer your-api-key" http://localhost:8080/health
```

#### Redis Connection Issues

```bash
# Test Redis connectivity
redis-cli ping

# Check Redis URL
echo $GACP_REDIS_URL

# Test connection with redis-cli
redis-cli -u "redis://localhost:6379" ping
```

#### File Watching Issues

```bash
# Check inotify limits (Linux)
cat /proc/sys/fs/inotify/max_user_watches

# Increase limits
echo 524288 | sudo tee /proc/sys/fs/inotify/max_user_watches

# Check file permissions
ls -la .gacp/
```

### Debug Mode

Enable debug logging for troubleshooting:

```bash
# Start with debug logging
gacp daemon start --log-level debug

# Or modify config file
sed -i 's/level: "info"/level: "debug"/' gacp-mcp.yaml
```

### Log Analysis

```bash
# Find errors
grep -i error /var/log/gacp-mcp.log

# Find warnings
grep -i warn /var/log/gacp-mcp.log

# Monitor real-time
tail -f /var/log/gacp-mcp.log | grep -E "(ERROR|WARN)"

# Analyze performance
grep "execution_time" /var/log/gacp-mcp.log | awk '{sum+=$NF; count++} END {print "Average:", sum/count}'
```

## Best Practices

### Security

1. **Always use authentication in production**
2. **Use strong, randomly generated API keys**
3. **Restrict CORS origins to your application domains**
4. **Use HTTPS in production environments**
5. **Regularly rotate API keys and JWT secrets**
6. **Monitor access logs for suspicious activity**

### Performance

1. **Use Redis for distributed caching**
2. **Configure appropriate cache TTL values**
3. **Monitor memory usage and cache hit rates**
4. **Use connection pooling for database connections**
5. **Implement rate limiting for API endpoints**
6. **Use WebSocket for real-time updates**

### Reliability

1. **Implement health checks and monitoring**
2. **Use systemd or similar for process management**
3. **Set up log rotation and archival**
4. **Implement graceful shutdown handling**
5. **Use load balancers for high availability**
6. **Regularly backup configuration and data**

### Development

1. **Use version control for configuration files**
2. **Implement automated testing for API endpoints**
3. **Use environment-specific configurations**
4. **Document API changes and breaking changes**
5. **Implement proper error handling in clients**
6. **Use structured logging for better debugging**

### Deployment

1. **Use containerization for consistent deployments**
2. **Implement blue-green deployments for zero downtime**
3. **Use configuration management tools**
4. **Set up automated monitoring and alerting**
5. **Implement proper backup and recovery procedures**
6. **Use infrastructure as code for deployment automation** 