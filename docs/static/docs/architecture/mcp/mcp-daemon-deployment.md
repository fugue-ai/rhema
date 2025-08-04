# Rhema MCP Daemon Deployment Guide


## Overview


This guide covers deploying the Rhema MCP Daemon in various environments, from local development to production cloud deployments.

## Docker Deployment


### Basic Dockerfile


```dockerfile
# Multi-stage build for smaller image


FROM rust:1.70 as builder

WORKDIR /app
COPY . .

# Build the application


RUN cargo build --release

# Runtime stage


FROM debian:bullseye-slim

# Install runtime dependencies


RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user


RUN useradd -r -s /bin/false rhema

# Copy binary from builder stage


COPY --from=builder /app/target/release/rhema /usr/local/bin/rhema

# Create necessary directories


RUN mkdir -p /app/.rhema /var/log/rhema /tmp && \
    chown -R rhema:rhema /app /var/log/rhema /tmp

WORKDIR /app
USER rhema

# Expose ports


EXPOSE 8080

# Health check


HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command


CMD ["rhema", "daemon", "start", "--host", "0.0.0.0", "--port", "8080"]
```

### Docker Compose Setup


```yaml
version: '3.8'

services:
  rhema-mcp:
    build: .
    ports:

      - "8080:8080"
    volumes:

      - ./.rhema:/app/.rhema:ro

      - ./config:/app/config:ro

      - rhema-logs:/var/log/rhema

      - /tmp/rhema-mcp.sock:/tmp/rhema-mcp.sock
    environment:

      - Rhema_API_KEY=${Rhema_API_KEY}

      - Rhema_REDIS_URL=redis://redis:6379

      - Rhema_LOG_LEVEL=info
    depends_on:

      - redis
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s

  redis:
    image: redis:7-alpine
    ports:

      - "6379:6379"
    volumes:

      - redis-data:/data
    restart: unless-stopped
    command: redis-server --appendonly yes

  nginx:
    image: nginx:alpine
    ports:

      - "80:80"

      - "443:443"
    volumes:

      - ./nginx.conf:/etc/nginx/nginx.conf:ro

      - ./ssl:/etc/nginx/ssl:ro
    depends_on:

      - rhema-mcp
    restart: unless-stopped

volumes:
  rhema-logs:
  redis-data:
```

### Nginx Configuration


```nginx
events {
    worker_connections 1024;
}

http {
    upstream rhema_backend {
        server rhema-mcp:8080;
    }

    # Rate limiting


    limit_req_zone $binary_remote_addr zone=rhema_api:10m rate=10r/s;

    server {
        listen 80;
        server_name your-domain.com;
        return 301 https://$server_name$request_uri;
    }

    server {
        listen 443 ssl http2;
        server_name your-domain.com;

        ssl_certificate /etc/nginx/ssl/cert.pem;
        ssl_certificate_key /etc/nginx/ssl/key.pem;
        ssl_protocols TLSv1.2 TLSv1.3;
        ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384;
        ssl_prefer_server_ciphers off;

        # Security headers


        add_header X-Frame-Options DENY;
        add_header X-Content-Type-Options nosniff;
        add_header X-XSS-Protection "1; mode=block";
        add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

        # Rate limiting


        limit_req zone=rhema_api burst=20 nodelay;

        location / {
            proxy_pass http://rhema_backend;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
            proxy_connect_timeout 30s;
            proxy_send_timeout 30s;
            proxy_read_timeout 30s;
        }

        # Health check endpoint


        location /health {
            proxy_pass http://rhema_backend/health;
            access_log off;
        }
    }
}
```

### Production Docker Compose


```yaml
version: '3.8'

services:
  rhema-mcp:
    image: rhema-mcp:latest
    deploy:
      replicas: 3
      resources:
        limits:
          cpus: '1.0'
          memory: 1G
        reservations:
          cpus: '0.5'
          memory: 512M
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3
        window: 120s
    ports:

      - "8080:8080"
    volumes:

      - ./.rhema:/app/.rhema:ro

      - ./config:/app/config:ro
    environment:

      - Rhema_API_KEY=${Rhema_API_KEY}

      - Rhema_REDIS_URL=redis://redis:6379

      - Rhema_LOG_LEVEL=info
    depends_on:

      - redis
    networks:

      - rhema-network

  redis:
    image: redis:7-alpine
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
    volumes:

      - redis-data:/data
    command: redis-server --appendonly yes --maxmemory 256mb --maxmemory-policy allkeys-lru
    networks:

      - rhema-network

  prometheus:
    image: prom/prometheus:latest
    ports:

      - "9090:9090"
    volumes:

      - ./prometheus.yml:/etc/prometheus/prometheus.yml:ro

      - prometheus-data:/prometheus
    command:

      - '--config.file=/etc/prometheus/prometheus.yml'

      - '--storage.tsdb.path=/prometheus'

      - '--web.console.libraries=/etc/prometheus/console_libraries'

      - '--web.console.templates=/etc/prometheus/consoles'

      - '--storage.tsdb.retention.time=200h'

      - '--web.enable-lifecycle'
    networks:

      - rhema-network

  grafana:
    image: grafana/grafana:latest
    ports:

      - "3000:3000"
    volumes:

      - grafana-data:/var/lib/grafana

      - ./grafana/dashboards:/etc/grafana/provisioning/dashboards:ro

      - ./grafana/datasources:/etc/grafana/provisioning/datasources:ro
    environment:

      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_PASSWORD}
    networks:

      - rhema-network

volumes:
  redis-data:
  prometheus-data:
  grafana-data:

networks:
  rhema-network:
    driver: overlay
```

## Kubernetes Deployment


### Namespace


```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: rhema
  labels:
    name: rhema
```

### ConfigMap


```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: rhema-config
  namespace: rhema
data:
  rhema-mcp.yaml: |
    host: "0.0.0.0"
    port: 8080
    redis_url: "redis://rhema-redis:6379"
    
    auth:
      enabled: true
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
```

### Secret


```yaml
apiVersion: v1
kind: Secret
metadata:
  name: rhema-secrets
  namespace: rhema
type: Opaque
data:
  api-key: <base64-encoded-api-key>
  jwt-secret: <base64-encoded-jwt-secret>
```

### Deployment


```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rhema-mcp
  namespace: rhema
  labels:
    app: rhema-mcp
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rhema-mcp
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  template:
    metadata:
      labels:
        app: rhema-mcp
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "8080"
        prometheus.io/path: "/metrics"
    spec:
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
        runAsGroup: 1000
        fsGroup: 1000
      containers:

      - name: rhema-mcp
        image: rhema-mcp:latest
        imagePullPolicy: Always
        ports:

        - containerPort: 8080
          name: http
        env:

        - name: Rhema_API_KEY
          valueFrom:
            secretKeyRef:
              name: rhema-secrets
              key: api-key

        - name: Rhema_JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: rhema-secrets
              key: jwt-secret

        - name: Rhema_REDIS_URL
          value: "redis://rhema-redis:6379"

        - name: Rhema_LOG_LEVEL
          value: "info"
        volumeMounts:

        - name: rhema-config
          mountPath: /app/.rhema
          readOnly: true

        - name: config-file
          mountPath: /etc/rhema/rhema-mcp.yaml
          subPath: rhema-mcp.yaml
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
        startupProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 10
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 30
      volumes:

      - name: rhema-config
        configMap:
          name: rhema-config

      - name: config-file
        configMap:
          name: rhema-config
```

### Service


```yaml
apiVersion: v1
kind: Service
metadata:
  name: rhema-mcp-service
  namespace: rhema
  labels:
    app: rhema-mcp
spec:
  selector:
    app: rhema-mcp
  ports:

  - protocol: TCP
    port: 80
    targetPort: 8080
    name: http
  type: ClusterIP
```

### Ingress


```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: rhema-mcp-ingress
  namespace: rhema
  annotations:
    kubernetes.io/ingress.class: "nginx"
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/rate-limit: "100"
    nginx.ingress.kubernetes.io/rate-limit-window: "1m"
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
spec:
  tls:

  - hosts:

    - api.your-domain.com
    secretName: rhema-mcp-tls
  rules:

  - host: api.your-domain.com
    http:
      paths:

      - path: /
        pathType: Prefix
        backend:
          service:
            name: rhema-mcp-service
            port:
              number: 80
```

### Horizontal Pod Autoscaler


```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: rhema-mcp-hpa
  namespace: rhema
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: rhema-mcp
  minReplicas: 3
  maxReplicas: 10
  metrics:

  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70

  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
  behavior:
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:

      - type: Percent
        value: 10
        periodSeconds: 60
    scaleUp:
      stabilizationWindowSeconds: 60
      policies:

      - type: Percent
        value: 50
        periodSeconds: 60
```

### Redis Deployment


```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: rhema-redis
  namespace: rhema
spec:
  serviceName: rhema-redis
  replicas: 3
  selector:
    matchLabels:
      app: rhema-redis
  template:
    metadata:
      labels:
        app: rhema-redis
    spec:
      containers:

      - name: redis
        image: redis:7-alpine
        ports:

        - containerPort: 6379
        command:

        - redis-server

        - /etc/redis/redis.conf
        volumeMounts:

        - name: redis-config
          mountPath: /etc/redis

        - name: redis-data
          mountPath: /data
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "256Mi"
            cpu: "200m"
  volumeClaimTemplates:

  - metadata:
      name: redis-data
    spec:
      accessModes: ["ReadWriteOnce"]
      resources:
        requests:
          storage: 1Gi
```

## Cloud Deployment


### AWS ECS Deployment


```yaml
# task-definition.json


{
  "family": "rhema-mcp",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "512",
  "memory": "1024",
  "executionRoleArn": "arn:aws:iam::123456789012:role/ecsTaskExecutionRole",
  "taskRoleArn": "arn:aws:iam::123456789012:role/rhema-mcp-task-role",
  "containerDefinitions": [
    {
      "name": "rhema-mcp",
      "image": "123456789012.dkr.ecr.us-west-2.amazonaws.com/rhema-mcp:latest",
      "portMappings": [
        {
          "containerPort": 8080,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "Rhema_REDIS_URL",
          "value": "redis://rhema-redis:6379"
        }
      ],
      "secrets": [
        {
          "name": "Rhema_API_KEY",
          "valueFrom": "arn:aws:secretsmanager:us-west-2:123456789012:secret:rhema-api-key"
        },
        {
          "name": "Rhema_JWT_SECRET",
          "valueFrom": "arn:aws:secretsmanager:us-west-2:123456789012:secret:rhema-jwt-secret"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/rhema-mcp",
          "awslogs-region": "us-west-2",
          "awslogs-stream-prefix": "ecs"
        }
      },
      "healthCheck": {
        "command": ["CMD-SHELL", "curl -f http://localhost:8080/health || exit 1"],
        "interval": 30,
        "timeout": 5,
        "retries": 3,
        "startPeriod": 60
      }
    }
  ]
}
```

### Google Cloud Run


```yaml
# cloudbuild.yaml


steps:

- name: 'gcr.io/cloud-builders/docker'
  args: ['build', '-t', 'gcr.io/$PROJECT_ID/rhema-mcp:$COMMIT_SHA', '.']

- name: 'gcr.io/cloud-builders/docker'
  args: ['push', 'gcr.io/$PROJECT_ID/rhema-mcp:$COMMIT_SHA']

- name: 'gcr.io/cloud-builders/gcloud'
  args:

  - 'run'

  - 'deploy'

  - 'rhema-mcp'

  - '--image'

  - 'gcr.io/$PROJECT_ID/rhema-mcp:$COMMIT_SHA'

  - '--region'

  - 'us-central1'

  - '--platform'

  - 'managed'

  - '--allow-unauthenticated'

  - '--port'

  - '8080'

  - '--memory'

  - '1Gi'

  - '--cpu'

  - '1'

  - '--max-instances'

  - '10'

  - '--set-env-vars'

  - 'Rhema_REDIS_URL=redis://rhema-redis:6379'

  - '--set-secrets'

  - 'Rhema_API_KEY=rhema-api-key:latest,Rhema_JWT_SECRET=rhema-jwt-secret:latest'

images:

- 'gcr.io/$PROJECT_ID/rhema-mcp:$COMMIT_SHA'
```

### Azure Container Instances


```yaml
# azure-deployment.yaml


apiVersion: 2019-12-01
location: eastus
name: rhema-mcp
properties:
  containers:

  - name: rhema-mcp
    properties:
      image: rhema-mcp:latest
      ports:

      - port: 8080
      environmentVariables:

      - name: Rhema_REDIS_URL
        value: "redis://rhema-redis:6379"

      - name: Rhema_LOG_LEVEL
        value: "info"
      resources:
        requests:
          memoryInGB: 1.0
          cpu: 1.0
        limits:
          memoryInGB: 2.0
          cpu: 2.0
  osType: Linux
  restartPolicy: Always
  ipAddress:
    type: Public
    ports:

    - protocol: tcp
      port: 8080
```

## Monitoring and Observability


### Prometheus Configuration


```yaml
# prometheus.yml


global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:

  - "rhema-rules.yml"

scrape_configs:

  - job_name: 'rhema-mcp'
    static_configs:

      - targets: ['rhema-mcp:8080']
    metrics_path: /metrics
    scrape_interval: 10s
    scrape_timeout: 5s

  - job_name: 'redis'
    static_configs:

      - targets: ['rhema-redis:6379']
    metrics_path: /metrics
```

### Grafana Dashboard


```json
{
  "dashboard": {
    "title": "Rhema MCP Daemon",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])",
            "legendFormat": "{{method}} {{endpoint}}"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          }
        ]
      },
      {
        "title": "Memory Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "process_resident_memory_bytes",
            "legendFormat": "Memory Usage"
          }
        ]
      },
      {
        "title": "Cache Hit Rate",
        "type": "stat",
        "targets": [
          {
            "expr": "rhema_cache_hit_rate",
            "legendFormat": "Cache Hit Rate"
          }
        ]
      }
    ]
  }
}
```

## Security Considerations


### Network Security


```yaml
# Network policies for Kubernetes


apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: rhema-mcp-network-policy
  namespace: rhema
spec:
  podSelector:
    matchLabels:
      app: rhema-mcp
  policyTypes:

  - Ingress

  - Egress
  ingress:

  - from:

    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:

    - protocol: TCP
      port: 8080
  egress:

  - to:

    - podSelector:
        matchLabels:
          app: rhema-redis
    ports:

    - protocol: TCP
      port: 6379

  - to: []
    ports:

    - protocol: TCP
      port: 53

    - protocol: UDP
      port: 53
```

### RBAC Configuration


```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: rhema-mcp
  namespace: rhema

---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: rhema-mcp-role
  namespace: rhema
rules:

- apiGroups: [""]
  resources: ["configmaps", "secrets"]
  verbs: ["get", "list", "watch"]

---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: rhema-mcp-rolebinding
  namespace: rhema
subjects:

- kind: ServiceAccount
  name: rhema-mcp
  namespace: rhema
roleRef:
  kind: Role
  name: rhema-mcp-role
  apiGroup: rbac.authorization.k8s.io
```

## Backup and Recovery


### Backup Script


```bash
#!/bin/bash


# Backup Rhema MCP Daemon configuration and data


BACKUP_DIR="/backup/rhema-mcp"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory


mkdir -p "$BACKUP_DIR/$DATE"

# Backup configuration


kubectl get configmap rhema-config -n rhema -o yaml > "$BACKUP_DIR/$DATE/configmap.yaml"
kubectl get secret rhema-secrets -n rhema -o yaml > "$BACKUP_DIR/$DATE/secret.yaml"

# Backup Redis data


kubectl exec -n rhema rhema-redis-0 -- redis-cli BGSAVE
sleep 10
kubectl cp rhema/rhema-redis-0:/data/dump.rdb "$BACKUP_DIR/$DATE/redis-dump.rdb"

# Backup logs


kubectl logs -n rhema -l app=rhema-mcp --tail=1000 > "$BACKUP_DIR/$DATE/logs.txt"

# Compress backup


tar -czf "$BACKUP_DIR/rhema-mcp-backup-$DATE.tar.gz" -C "$BACKUP_DIR" "$DATE"

# Clean up old backups (keep last 7 days)


find "$BACKUP_DIR" -name "rhema-mcp-backup-*.tar.gz" -mtime +7 -delete

echo "Backup completed: $BACKUP_DIR/rhema-mcp-backup-$DATE.tar.gz"
```

### Recovery Script


```bash
#!/bin/bash


# Recovery script for Rhema MCP Daemon


BACKUP_FILE="$1"

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: $0 <backup-file>"
    exit 1
fi

# Extract backup


tar -xzf "$BACKUP_FILE"
BACKUP_DIR=$(basename "$BACKUP_FILE" .tar.gz)

# Restore configuration


kubectl apply -f "$BACKUP_DIR/configmap.yaml"
kubectl apply -f "$BACKUP_DIR/secret.yaml"

# Restart deployment to pick up new configuration


kubectl rollout restart deployment/rhema-mcp -n rhema

# Restore Redis data


kubectl cp "$BACKUP_DIR/redis-dump.rdb" rhema/rhema-redis-0:/data/dump.rdb
kubectl exec -n rhema rhema-redis-0 -- redis-cli BGREWRITEAOF

echo "Recovery completed"
```

## Performance Tuning


### Resource Optimization


```yaml
# High-performance configuration


apiVersion: apps/v1
kind: Deployment
metadata:
  name: rhema-mcp
spec:
  template:
    spec:
      containers:

      - name: rhema-mcp
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
        env:

        - name: Rhema_CACHE_MAX_SIZE
          value: "50000"

        - name: Rhema_CACHE_TTL_SECONDS
          value: "7200"

        - name: Rhema_WATCHER_DEBOUNCE_MS
          value: "50"
```

### Load Testing


```bash
#!/bin/bash


# Load testing script


DAEMON_URL="http://localhost:8080"
CONCURRENT_USERS=100
DURATION=300

# Install hey if not available


if ! command -v hey &> /dev/null; then
    go install github.com/rakyll/hey@latest
fi

echo "Starting load test..."
echo "URL: $DAEMON_URL"
echo "Concurrent users: $CONCURRENT_USERS"
echo "Duration: $DURATION seconds"

# Health check load test


hey -n 1000 -c $CONCURRENT_USERS -z ${DURATION}s "$DAEMON_URL/health"

# Query load test


hey -n 1000 -c $CONCURRENT_USERS -z ${DURATION}s -m POST \
    -H "Content-Type: application/json" \
    -d '{"query": "SELECT * FROM scopes"}' \
    "$DAEMON_URL/query"

echo "Load test completed"
``` 