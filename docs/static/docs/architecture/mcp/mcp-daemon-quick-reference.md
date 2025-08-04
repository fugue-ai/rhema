# Rhema MCP Daemon Quick Reference


## Quick Start Commands


### Installation


```bash
# Build from source


cargo build --release
cargo install --path .

# Or download binary


curl -L https://github.com/fugue-ai/rhema/releases/latest/download/rhema-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv rhema /usr/local/bin/
```

### Basic Usage


```bash
# Start daemon with defaults


rhema daemon start

# Start with custom config


rhema daemon start --config rhema-mcp.yaml

# Start with command line options


rhema daemon start --host 0.0.0.0 --port 8080 --auth --api-key "your-key"

# Stop daemon


rhema daemon stop

# Check status


rhema daemon status
```

## Configuration Quick Reference


### Minimal Configuration


```yaml
host: "127.0.0.1"
port: 8080
auth:
  enabled: false
watcher:
  enabled: true
  watch_dirs: [".rhema"]
cache:
  memory_enabled: true
  ttl_seconds: 3600
```

### Production Configuration


```yaml
host: "0.0.0.0"
port: 8080
unix_socket: "/var/run/rhema-mcp.sock"
redis_url: "redis://redis:6379"

auth:
  enabled: true
  api_key: "${Rhema_API_KEY}"
  jwt_secret: "${Rhema_JWT_SECRET}"
  allowed_origins: ["https://your-app.example.com"]

watcher:
  enabled: true
  watch_dirs: [".rhema", "config"]
  file_patterns: ["*.yaml", "*.yml"]
  debounce_ms: 100

cache:
  memory_enabled: true
  redis_enabled: true
  ttl_seconds: 3600
  max_size: 10000

logging:
  level: "info"
  structured: true
  file: "/var/log/rhema-mcp.log"
```

## API Quick Reference


### Health Check


```bash
curl http://localhost:8080/health
```

### List Scopes


```bash
curl http://localhost:8080/scopes
```

### Execute Query


```bash
curl -X POST http://localhost:8080/query \
  -H "Content-Type: application/json" \
  -d '{"query": "SELECT * FROM scopes"}'
```

### Get Resource


```bash
curl http://localhost:8080/resources/rhema%3A//scopes/test/scope.yaml
```

### With Authentication


```bash
curl -H "Authorization: Bearer your-api-key" \
  http://localhost:8080/scopes
```

## WebSocket Quick Reference


### Connect


```javascript
const ws = new WebSocket('ws://localhost:8081/ws');
```

### Send Request


```javascript
ws.send(JSON.stringify({
  jsonrpc: "2.0",
  id: 1,
  method: "system/health",
  params: {}
}));
```

### Subscribe to Changes


```javascript
ws.send(JSON.stringify({
  jsonrpc: "2.0",
  id: 2,
  method: "resources/subscribe",
  params: { uri: "rhema://scopes/test" }
}));
```

## Unix Socket Quick Reference


### Connect


```bash
nc -U /tmp/rhema-mcp.sock
```

### Send Request


```bash
echo '{"jsonrpc": "2.0", "id": 1, "method": "system/health", "params": {}}' | nc -U /tmp/rhema-mcp.sock
```

## Docker Quick Reference


### Build Image


```bash
docker build -t rhema-mcp .
```

### Run Container


```bash
docker run -d \
  --name rhema-mcp \
  -p 8080:8080 \
  -v $(pwd)/.rhema:/app/.rhema:ro \
  -e Rhema_API_KEY=your-key \
  rhema-mcp
```

### Docker Compose


```bash
# Start


docker-compose up -d

# Stop


docker-compose down

# View logs


docker-compose logs -f rhema-mcp
```

## Kubernetes Quick Reference


### Deploy


```bash
kubectl apply -f k8s/
```

### Check Status


```bash
kubectl get pods -n rhema
kubectl logs -f deployment/rhema-mcp -n rhema
```

### Scale


```bash
kubectl scale deployment rhema-mcp --replicas=5 -n rhema
```

### Port Forward


```bash
kubectl port-forward service/rhema-mcp-service 8080:80 -n rhema
```

## Environment Variables


| Variable | Description | Default |
|----------|-------------|---------|
| `Rhema_API_KEY` | API key for authentication | None |
| `Rhema_JWT_SECRET` | JWT secret for token generation | None |
| `Rhema_REDIS_URL` | Redis connection URL | None |
| `Rhema_LOG_LEVEL` | Log level (trace, debug, info, warn, error) | info |
| `Rhema_LOG_FILE` | Log file path | None |
| `Rhema_HOST` | Host to bind to | 127.0.0.1 |
| `Rhema_PORT` | Port to bind to | 8080 |

## Common Queries


### List All Scopes


```sql
SELECT * FROM scopes
```

### Find Service Scopes


```sql
SELECT * FROM scopes WHERE type = 'service'
```

### Search Knowledge


```sql
SELECT * FROM knowledge WHERE content LIKE '%architecture%'
```

### Find High Priority Todos


```sql
SELECT * FROM todos WHERE priority = 'high' AND status != 'completed'
```

### Recent Decisions


```sql
SELECT * FROM decisions WHERE created_at > '2024-01-01'
```

## Troubleshooting Quick Reference


### Daemon Won't Start


```bash
# Check if port is in use


sudo lsof -i :8080

# Check permissions


ls -la /tmp/rhema-mcp.sock

# Check logs


tail -f /var/log/rhema-mcp.log
```

### Authentication Issues


```bash
# Test API key


curl -H "Authorization: Bearer your-key" http://localhost:8080/health

# Check environment variable


echo $Rhema_API_KEY
```

### Redis Connection Issues


```bash
# Test Redis


redis-cli ping

# Check Redis URL


echo $Rhema_REDIS_URL
```

### File Watching Issues


```bash
# Check inotify limits


cat /proc/sys/fs/inotify/max_user_watches

# Increase limits


echo 524288 | sudo tee /proc/sys/fs/inotify/max_user_watches
```

### Performance Issues


```bash
# Check memory usage


curl http://localhost:8080/health | jq '.memory_usage'

# Check cache hit rate


curl http://localhost:8080/health | jq '.cache_hit_rate'

# Monitor connections


watch -n 5 'curl -s http://localhost:8080/health | jq ".connections"'
```

## Monitoring Commands


### Health Check


```bash
# Basic health


curl -f http://localhost:8080/health || exit 1

# Detailed health


curl http://localhost:8080/health | jq '.'

# Specific metrics


curl http://localhost:8080/health | jq '.memory_usage'
curl http://localhost:8080/health | jq '.cache_hit_rate'
```

### Statistics


```bash
# Get stats


curl http://localhost:8080/stats | jq '.'

# Cache stats


curl http://localhost:8080/stats | jq '.cache_stats'

# File stats


curl http://localhost:8080/stats | jq '.file_stats'
```

### Log Monitoring


```bash
# Follow logs


tail -f /var/log/rhema-mcp.log

# Search for errors


grep ERROR /var/log/rhema-mcp.log

# Search for warnings


grep WARN /var/log/rhema-mcp.log

# Monitor real-time


tail -f /var/log/rhema-mcp.log | grep -E "(ERROR|WARN)"
```

## Security Quick Reference


### Generate API Key


```bash
# Generate random API key


openssl rand -hex 32

# Generate JWT secret


openssl rand -base64 64
```

### Secure Configuration


```yaml
# Production security settings


host: "127.0.0.1"  # Bind to localhost only
auth:
  enabled: true
  api_key: "${Rhema_API_KEY}"
  allowed_origins: ["https://your-app.example.com"]
```

### Network Security


```bash
# Firewall rules (iptables)


sudo iptables -A INPUT -p tcp --dport 8080 -s 192.168.1.0/24 -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 8080 -j DROP

# Firewall rules (ufw)


sudo ufw allow from 192.168.1.0/24 to any port 8080
```

## Backup and Recovery


### Backup Configuration


```bash
# Backup config


cp rhema-mcp.yaml rhema-mcp.yaml.backup

# Backup with timestamp


cp rhema-mcp.yaml rhema-mcp.yaml.$(date +%Y%m%d_%H%M%S)
```

### Backup Data


```bash
# Backup Redis data


redis-cli BGSAVE

# Backup logs


cp /var/log/rhema-mcp.log /backup/rhema-mcp.log.$(date +%Y%m%d)
```

### Restore


```bash
# Restore config


cp rhema-mcp.yaml.backup rhema-mcp.yaml

# Restart daemon


rhema daemon restart
```

## Performance Tuning


### Memory Optimization


```yaml
# High memory configuration


cache:
  max_size: 50000
  ttl_seconds: 7200

logging:
  level: "warn"  # Less verbose
```

### CPU Optimization


```yaml
# High performance configuration


watcher:
  debounce_ms: 50  # Faster response

cache:
  memory_enabled: true
  redis_enabled: true
```

### Network Optimization


```yaml
# Network optimization


host: "0.0.0.0"
port: 8080
unix_socket: "/var/run/rhema-mcp.sock"  # For local access
```

## Common Error Messages


| Error | Cause | Solution |
|-------|-------|----------|
| `Port already in use` | Another service using port 8080 | Change port or stop conflicting service |
| `Permission denied` | Unix socket permissions | `chmod 660 /tmp/rhema-mcp.sock` |
| `Redis connection failed` | Redis not running or wrong URL | Start Redis or check URL |
| `Authentication failed` | Invalid API key | Check API key configuration |
| `File not found` | Missing .rhema directory | Create .rhema directory and scope files |
| `Memory limit exceeded` | Cache too large | Reduce cache size or increase memory |

## Development Quick Reference


### Debug Mode


```bash
# Start with debug logging


rhema daemon start --log-level debug

# Or modify config


sed -i 's/level: "info"/level: "debug"/' rhema-mcp.yaml
```

### Test API


```bash
# Test all endpoints


curl http://localhost:8080/health
curl http://localhost:8080/info
curl http://localhost:8080/scopes
curl http://localhost:8080/stats
```

### Load Testing


```bash
# Simple load test


for i in {1..100}; do
  curl -s http://localhost:8080/health > /dev/null &
done
wait

# Using hey (install with: go install github.com/rakyll/hey@latest)


hey -n 1000 -c 10 http://localhost:8080/health
```

### Profile Performance


```bash
# CPU profiling


curl http://localhost:8080/debug/pprof/profile

# Memory profiling


curl http://localhost:8080/debug/pprof/heap
``` 