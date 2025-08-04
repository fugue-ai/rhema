# Local Development Environment Setup


This guide will help you set up a complete local development environment for Rhema development. This includes Docker containers, databases, environment configuration, and development tools.

## Prerequisites


- [Docker](https://docs.docker.com/get-docker/) and Docker Compose

- [Rhema CLI](../README.md#installation) installed

- [Rust toolchain](rust-setup.md) for development

- [Git](git-setup.md) configured

## Docker Setup


### 1. Development Environment


Create `docker-compose.yml` for the development environment:

```yaml
version: '3.8'

services:
  # PostgreSQL database for testing


  postgres:
    image: postgres:15-alpine
    container_name: rhema-postgres
    environment:
      POSTGRES_DB: rhema_dev
      POSTGRES_USER: rhema_user
      POSTGRES_PASSWORD: rhema_password
    ports:

      - "5432:5432"
    volumes:

      - postgres_data:/var/lib/postgresql/data

      - ./scripts/init-db.sql:/docker-entrypoint-initdb.d/init-db.sql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U rhema_user -d rhema_dev"]
      interval: 10s
      timeout: 5s
      retries: 5

  # Redis for caching


  redis:
    image: redis:7-alpine
    container_name: rhema-redis
    ports:

      - "6379:6379"
    volumes:

      - redis_data:/data
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5

  # MinIO for S3-compatible storage


  minio:
    image: minio/minio:latest
    container_name: rhema-minio
    environment:
      MINIO_ROOT_USER: rhema_user
      MINIO_ROOT_PASSWORD: rhema_password
    ports:

      - "9000:9000"

      - "9001:9001"
    volumes:

      - minio_data:/data
    command: server /data --console-address ":9001"
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:9000/minio/health/live"]
      interval: 30s
      timeout: 20s
      retries: 3

  # Elasticsearch for search functionality


  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:8.8.0
    container_name: rhema-elasticsearch
    environment:

      - discovery.type=single-node

      - xpack.security.enabled=false

      - "ES_JAVA_OPTS=-Xms512m -Xmx512m"
    ports:

      - "9200:9200"
    volumes:

      - elasticsearch_data:/usr/share/elasticsearch/data
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:9200/_cluster/health || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 5

  # Kibana for Elasticsearch visualization


  kibana:
    image: docker.elastic.co/kibana/kibana:8.8.0
    container_name: rhema-kibana
    environment:

      - ELASTICSEARCH_HOSTS=http://elasticsearch:9200
    ports:

      - "5601:5601"
    depends_on:
      elasticsearch:
        condition: service_healthy
    healthcheck:
      test: ["CMD-SHELL", "curl -f http://localhost:5601/api/status || exit 1"]
      interval: 30s
      timeout: 10s
      retries: 5

  # Prometheus for metrics


  prometheus:
    image: prom/prometheus:latest
    container_name: rhema-prometheus
    ports:

      - "9090:9090"
    volumes:

      - ./config/prometheus.yml:/etc/prometheus/prometheus.yml

      - prometheus_data:/prometheus
    command:

      - '--config.file=/etc/prometheus/prometheus.yml'

      - '--storage.tsdb.path=/prometheus'

      - '--web.console.libraries=/etc/prometheus/console_libraries'

      - '--web.console.templates=/etc/prometheus/consoles'

      - '--storage.tsdb.retention.time=200h'

      - '--web.enable-lifecycle'

  # Grafana for metrics visualization


  grafana:
    image: grafana/grafana:latest
    container_name: rhema-grafana
    environment:

      - GF_SECURITY_ADMIN_PASSWORD=rhema_password
    ports:

      - "3000:3000"
    volumes:

      - grafana_data:/var/lib/grafana

      - ./config/grafana/dashboards:/etc/grafana/provisioning/dashboards

      - ./config/grafana/datasources:/etc/grafana/provisioning/datasources
    depends_on:
      prometheus:
        condition: service_healthy

volumes:
  postgres_data:
  redis_data:
  minio_data:
  elasticsearch_data:
  prometheus_data:
  grafana_data:
```

### 2. Development Scripts


Create `scripts/dev-setup.sh`:

```bash
#!/bin/bash


# Development environment setup script


set -e

echo "Setting up Rhema development environment..."

# Check if Docker is running


if ! docker info > /dev/null 2>&1; then
    echo "Error: Docker is not running. Please start Docker and try again."
    exit 1
fi

# Create necessary directories


mkdir -p config/prometheus
mkdir -p config/grafana/dashboards
mkdir -p config/grafana/datasources
mkdir -p scripts

# Create Prometheus configuration


cat > config/prometheus.yml << 'EOF'
global:
  scrape_interval: 15s

scrape_configs:

  - job_name: 'rhema'
    static_configs:

      - targets: ['host.docker.internal:8080']
    metrics_path: '/metrics'
EOF

# Create Grafana datasource configuration


cat > config/grafana/datasources/prometheus.yml << 'EOF'
apiVersion: 1

datasources:

  - name: Prometheus
    type: prometheus
    access: proxy
    url: http://prometheus:9090
    isDefault: true
EOF

# Create database initialization script


cat > scripts/init-db.sql << 'EOF'
-- Initialize Rhema development database

-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- Create tables for Rhema testing
CREATE TABLE IF NOT EXISTS rhema_test_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    scope_name VARCHAR(255) NOT NULL,
    data_type VARCHAR(50) NOT NULL,
    content JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_rhema_test_data_scope ON rhema_test_data(scope_name);
CREATE INDEX IF NOT EXISTS idx_rhema_test_data_type ON rhema_test_data(data_type);
CREATE INDEX IF NOT EXISTS idx_rhema_test_data_content ON rhema_test_data USING GIN(content);

-- Create updated_at trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_rhema_test_data_updated_at
    BEFORE UPDATE ON rhema_test_data
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
EOF

# Start services


echo "Starting development services..."
docker-compose up -d

# Wait for services to be healthy


echo "Waiting for services to be ready..."
docker-compose run --rm --entrypoint "sh -c 'until pg_isready -h postgres -U rhema_user -d rhema_dev; do sleep 1; done'" postgres

echo "Development environment is ready!"
echo ""
echo "Services:"
echo "  PostgreSQL: localhost:5432"
echo "  Redis: localhost:6379"
echo "  MinIO: localhost:9000 (API), localhost:9001 (Console)"
echo "  Elasticsearch: localhost:9200"
echo "  Kibana: localhost:5601"
echo "  Prometheus: localhost:9090"
echo "  Grafana: localhost:3000 (admin/rhema_password)"
echo ""
echo "To stop services: docker-compose down"
echo "To view logs: docker-compose logs -f"
```

Make it executable:
```bash
chmod +x scripts/dev-setup.sh
```

### 3. Environment Configuration


Create `.env.development`:

```bash
# Development environment variables


# Database


DATABASE_URL=postgresql://rhema_user:rhema_password@localhost:5432/rhema_dev
DATABASE_POOL_SIZE=10
DATABASE_TIMEOUT=30

# Redis


REDIS_URL=redis://localhost:6379
REDIS_POOL_SIZE=10

# MinIO/S3


S3_ENDPOINT=http://localhost:9000
S3_ACCESS_KEY=rhema_user
S3_SECRET_KEY=rhema_password
S3_BUCKET=rhema-dev
S3_REGION=us-east-1

# Elasticsearch


ELASTICSEARCH_URL=http://localhost:9200
ELASTICSEARCH_INDEX=rhema-dev

# Application


APP_ENV=development
APP_DEBUG=true
APP_LOG_LEVEL=debug
APP_PORT=8080
APP_HOST=0.0.0.0

# Security


JWT_SECRET=your-jwt-secret-key-here
ENCRYPTION_KEY=your-encryption-key-here

# Monitoring


PROMETHEUS_ENABLED=true
PROMETHEUS_PORT=9090
GRAFANA_URL=http://localhost:3000

# Testing


TEST_DATABASE_URL=postgresql://rhema_user:rhema_password@localhost:5432/rhema_test
TEST_REDIS_URL=redis://localhost:6379/1
```

Create `.env.test`:

```bash
# Test environment variables


# Database


DATABASE_URL=postgresql://rhema_user:rhema_password@localhost:5432/rhema_test
DATABASE_POOL_SIZE=5
DATABASE_TIMEOUT=10

# Redis


REDIS_URL=redis://localhost:6379/1
REDIS_POOL_SIZE=5

# Application


APP_ENV=test
APP_DEBUG=false
APP_LOG_LEVEL=warn
APP_PORT=8081
APP_HOST=127.0.0.1

# Testing


TEST_TIMEOUT=30
TEST_PARALLEL=true
```

## Development Tools


### 1. Development Scripts


Create `scripts/dev.sh`:

```bash
#!/bin/bash


# Development helper script


set -e

case "$1" in
    "start")
        echo "Starting development environment..."
        docker-compose up -d
        ;;
    "stop")
        echo "Stopping development environment..."
        docker-compose down
        ;;
    "restart")
        echo "Restarting development environment..."
        docker-compose restart
        ;;
    "logs")
        docker-compose logs -f
        ;;
    "build")
        echo "Building Rhema CLI..."
        cargo build
        ;;
    "test")
        echo "Running tests..."
        cargo test
        ;;
    "test-integration")
        echo "Running integration tests..."
        cargo test --test '*'
        ;;
    "test-e2e")
        echo "Running end-to-end tests..."
        ./scripts/test-e2e.sh
        ;;
    "lint")
        echo "Running linters..."
        cargo fmt -- --check
        cargo clippy -- -D warnings
        ;;
    "clean")
        echo "Cleaning up..."
        cargo clean
        docker-compose down -v
        ;;
    "reset-db")
        echo "Resetting database..."
        docker-compose exec postgres psql -U rhema_user -d rhema_dev -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
        docker-compose exec postgres psql -U rhema_user -d rhema_dev -f /docker-entrypoint-initdb.d/init-db.sql
        ;;
    "seed")
        echo "Seeding test data..."
        ./scripts/seed-test-data.sh
        ;;
    "monitor")
        echo "Opening monitoring dashboards..."
        open http://localhost:3000  # Grafana
        open http://localhost:9090  # Prometheus
        open http://localhost:5601  # Kibana
        ;;
    *)
        echo "Usage: $0 {start|stop|restart|logs|build|test|test-integration|test-e2e|lint|clean|reset-db|seed|monitor}"
        exit 1
        ;;
esac
```

### 2. Test Data Seeding


Create `scripts/seed-test-data.sh`:

```bash
#!/bin/bash


# Seed test data for development


set -e

echo "Seeding test data..."

# Create test scopes


mkdir -p test-data/scopes

# Create user-service scope


cat > test-data/scopes/user-service/.rhema/rhema.yaml << 'EOF'
rhema:
  version: "1.0.0"
  scope:
    type: "service"
    name: "user-service"
    description: "User management and authentication service"
    boundaries:
      includes: ["src/**", "tests/**"]
    dependencies:
      parent: "../shared"
    responsibilities:

      - "User authentication"

      - "User profile management"

      - "Password reset functionality"
EOF

# Create knowledge base


cat > test-data/scopes/user-service/.rhema/knowledge.yaml << 'EOF'
insights:
  performance:

    - finding: "Database queries are not optimized"
      impact: "High latency on user operations"
      solution: "Add database indexes and query optimization"
      confidence: "high"
      evidence: ["Query logs", "Performance metrics"]
      related_files: ["src/repository.rs", "migrations/"]
      recorded_at: "2024-01-15T10:00:00Z"
EOF

# Create todos


cat > test-data/scopes/user-service/.rhema/todos.yaml << 'EOF'
todos:

  - id: "todo-001"
    title: "Implement rate limiting"
    description: "Add rate limiting to prevent abuse"
    status: in_progress
    priority: high
    assigned_to: "alice"
    created_at: "2024-01-15T09:00:00Z"
    tags: ["security", "performance"]
EOF

# Create decisions


cat > test-data/scopes/user-service/.rhema/decisions.yaml << 'EOF'
decisions:

  - id: "decision-001"
    title: "Use PostgreSQL for user service"
    description: "Chosen for ACID compliance and existing team expertise"
    status: approved
    rationale: "MongoDB lacks ACID transactions needed for user data integrity"
    alternatives_considered: ["MongoDB", "MySQL"]
    impact: "Affects user-service, auth-service, and payment-service"
    decided_at: "2024-01-15T08:00:00Z"
EOF

echo "Test data seeded successfully!"
```

### 3. End-to-End Testing


Create `scripts/test-e2e.sh`:

```bash
#!/bin/bash


# End-to-end testing script


set -e

echo "Running end-to-end tests..."

# Ensure services are running


docker-compose up -d

# Wait for services to be ready


echo "Waiting for services..."
sleep 30

# Test database connection


echo "Testing database connection..."
docker-compose exec postgres psql -U rhema_user -d rhema_dev -c "SELECT 1;" > /dev/null

# Test Redis connection


echo "Testing Redis connection..."
docker-compose exec redis redis-cli ping > /dev/null

# Test Elasticsearch connection


echo "Testing Elasticsearch connection..."
curl -f http://localhost:9200/_cluster/health > /dev/null

# Test Rhema CLI


echo "Testing Rhema CLI..."
cargo build --release
./target/release/rhema --version

# Initialize test scope


echo "Testing Rhema scope initialization..."
mkdir -p test-e2e
cd test-e2e
../target/release/rhema init --scope-type service --scope-name test-service

# Validate Rhema files


echo "Testing Rhema validation..."
../target/release/rhema validate --recursive

# Test queries


echo "Testing Rhema queries..."
../target/release/rhema query "todos WHERE status='pending'"

# Cleanup


cd ..
rm -rf test-e2e

echo "End-to-end tests completed successfully!"
```

## Monitoring and Debugging


### 1. Health Checks


Create `scripts/health-check.sh`:

```bash
#!/bin/bash


# Health check script for development services


set -e

echo "Checking development services health..."

# Check PostgreSQL


echo "Checking PostgreSQL..."
if docker-compose exec postgres pg_isready -U rhema_user -d rhema_dev > /dev/null; then
    echo "✓ PostgreSQL is healthy"
else
    echo "✗ PostgreSQL is not healthy"
    exit 1
fi

# Check Redis


echo "Checking Redis..."
if docker-compose exec redis redis-cli ping > /dev/null; then
    echo "✓ Redis is healthy"
else
    echo "✗ Redis is not healthy"
    exit 1
fi

# Check Elasticsearch


echo "Checking Elasticsearch..."
if curl -f http://localhost:9200/_cluster/health > /dev/null; then
    echo "✓ Elasticsearch is healthy"
else
    echo "✗ Elasticsearch is not healthy"
    exit 1
fi

# Check MinIO


echo "Checking MinIO..."
if curl -f http://localhost:9000/minio/health/live > /dev/null; then
    echo "✓ MinIO is healthy"
else
    echo "✗ MinIO is not healthy"
    exit 1
fi

# Check Prometheus


echo "Checking Prometheus..."
if curl -f http://localhost:9090/-/healthy > /dev/null; then
    echo "✓ Prometheus is healthy"
else
    echo "✗ Prometheus is not healthy"
    exit 1
fi

# Check Grafana


echo "Checking Grafana..."
if curl -f http://localhost:3000/api/health > /dev/null; then
    echo "✓ Grafana is healthy"
else
    echo "✗ Grafana is not healthy"
    exit 1
fi

echo "All services are healthy!"
```

### 2. Performance Monitoring


Create `scripts/monitor-performance.sh`:

```bash
#!/bin/bash


# Performance monitoring script


set -e

echo "Starting performance monitoring..."

# Monitor system resources


echo "System resources:"
docker stats --no-stream

# Monitor database performance


echo "Database performance:"
docker-compose exec postgres psql -U rhema_user -d rhema_dev -c "
SELECT 
    schemaname,
    tablename,
    attname,
    n_distinct,
    correlation
FROM pg_stats 
WHERE schemaname = 'public'
ORDER BY n_distinct DESC;
"

# Monitor Redis performance


echo "Redis performance:"
docker-compose exec redis redis-cli info memory

# Monitor Elasticsearch performance


echo "Elasticsearch performance:"
curl -s http://localhost:9200/_cluster/stats | jq '.indices'

echo "Performance monitoring completed!"
```

## Troubleshooting


### Common Issues


1. **Port conflicts**: Check if ports are already in use

2. **Memory issues**: Increase Docker memory allocation

3. **Permission issues**: Check file permissions on scripts

4. **Network issues**: Ensure Docker network is working

5. **Service startup failures**: Check service logs

### Debugging Commands


```bash
# View service logs


docker-compose logs -f [service-name]

# Access service shell


docker-compose exec [service-name] sh

# Check service status


docker-compose ps

# Restart specific service


docker-compose restart [service-name]

# View resource usage


docker stats

# Clean up everything


docker-compose down -v --remove-orphans
```

## Next Steps


1. **Start the environment**: Run `./scripts/dev-setup.sh`

2. **Run tests**: Use `./scripts/dev.sh test`

3. **Monitor services**: Use `./scripts/dev.sh monitor`

4. **Seed test data**: Use `./scripts/dev.sh seed`

5. **Debug issues**: Use the troubleshooting commands

For more information, see the [Rust Development Setup](rust-setup.md) and [Git Workflow Setup](git-setup.md) guides. 