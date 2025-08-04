# Infrastructure Directory

This directory contains all Docker-related infrastructure files for the Rhema project.

## 📁 Directory Structure

```
infra/
├── README.md              # This file
├── Dockerfile             # Main Rhema application Dockerfile
├── Dockerfile.mcp         # MCP daemon Dockerfile
└── docker-compose.yml     # Complete development/production environment
```

## 🐳 Docker Files

### Dockerfile
The main Dockerfile for building the Rhema application. Features:
- Multi-stage build for optimized image size
- Rust 1.75 slim base image
- Production-ready runtime with security best practices
- Health checks and proper user permissions

### Dockerfile.mcp
Dockerfile for building the MCP (Model Context Protocol) daemon:
- Separate build for MCP-specific functionality
- Optimized for daemon deployment
- Includes health checks and monitoring

### docker-compose.yml
Complete development and production environment including:
- **Rhema Service**: Main application container
- **Redis**: Caching and session storage
- **Prometheus**: Metrics collection
- **Grafana**: Monitoring dashboards
- **Jaeger**: Distributed tracing
- **Elasticsearch**: Log aggregation
- **Kibana**: Log visualization

## 🚀 Quick Start

### Prerequisites
- Docker and Docker Compose installed
- At least 4GB of available memory for all services

### Development Environment
```bash
# Start all services
cd infra
docker-compose up -d

# View logs
docker-compose logs -f

# Stop all services
docker-compose down
```

### Production Deployment
```bash
# Build and start production services
cd infra
docker-compose -f docker-compose.yml up -d --build

# Scale services if needed
docker-compose up -d --scale rhema=3
```

## 🔧 Service Configuration

### Rhema Service
- **Port**: 8080
- **Memory**: 1GB limit, 512MB reserved
- **CPU**: 0.5 cores limit, 0.25 cores reserved
- **Volumes**: Data, cache, logs, and repos directories

### Monitoring Stack
- **Prometheus**: Port 9090 (metrics collection)
- **Grafana**: Port 3000 (dashboards, admin/admin)
- **Jaeger**: Port 16686 (tracing UI)

### Data Storage
- **Redis**: Port 6379 (caching)
- **Elasticsearch**: Port 9200 (logs)
- **Kibana**: Port 5601 (log visualization)

## 📊 Monitoring

### Access Points
- **Grafana**: http://localhost:3000 (admin/admin)
- **Prometheus**: http://localhost:9090
- **Jaeger**: http://localhost:16686
- **Kibana**: http://localhost:5601
- **Elasticsearch**: http://localhost:9200

### Health Checks
All services include health checks:
```bash
# Check service health
docker-compose ps

# View health check logs
docker-compose logs rhema | grep health
```

## 🔍 Troubleshooting

### Common Issues

#### Memory Issues
```bash
# Check memory usage
docker stats

# Increase Docker memory allocation in Docker Desktop
# Settings > Resources > Memory: 4GB+
```

#### Port Conflicts
```bash
# Check what's using a port
lsof -i :8080

# Use different ports in docker-compose.yml
ports:
  - "8081:8080"  # Map host port 8081 to container port 8080
```

#### Permission Issues
```bash
# Fix file permissions
sudo chown -R $USER:$USER .

# Add user to docker group
sudo usermod -aG docker $USER
# Log out and back in
```

### Debugging Commands
```bash
# View service logs
docker-compose logs -f rhema

# Execute commands in containers
docker-compose exec rhema rhema --help

# Check service status
docker-compose ps

# Restart specific service
docker-compose restart rhema
```

## 🛠️ Development

### Building Images
```bash
# Build main application
docker build -f infra/Dockerfile -t rhema:latest .

# Build MCP daemon
docker build -f infra/Dockerfile.mcp -t rhema-mcp:latest .

# Build all services
docker-compose build
```

### Custom Configuration
Create a `.env` file in the infra directory:
```bash
# .env
RHEMA_ENV=development
RUST_LOG=debug
REDIS_URL=redis://redis:6379
```

### Adding New Services
1. Add service definition to `docker-compose.yml`
2. Update network configuration if needed
3. Add health checks and resource limits
4. Document in this README

## 🔒 Security

### Best Practices
- All containers run as non-root users
- Minimal base images (debian:bookworm-slim)
- Regular security updates
- Resource limits to prevent DoS
- Health checks for monitoring

### Network Security
- Services communicate via internal Docker network
- Only necessary ports exposed to host
- No sensitive data in environment variables

## 📈 Performance

### Resource Optimization
- Multi-stage builds for smaller images
- Shared volumes for data persistence
- Efficient base images
- Resource limits to prevent resource exhaustion

### Monitoring
- Prometheus metrics collection
- Grafana dashboards for visualization
- Jaeger for distributed tracing
- Elasticsearch/Kibana for log analysis

## 🔄 Maintenance

### Regular Tasks
```bash
# Update base images
docker-compose pull

# Clean up unused resources
docker system prune -f

# Backup volumes
docker run --rm -v rhema_data:/data -v $(pwd):/backup alpine tar czf /backup/rhema-data.tar.gz -C /data .
```

### Updates
1. Update Dockerfile base images
2. Test with `docker-compose build`
3. Update this documentation
4. Commit changes with descriptive messages

## 📚 Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Rhema Architecture Documentation](../ARCHITECTURE.md)
- [Development Setup Guide](../docs/development-setup/development/local-setup.md)

---

**Note**: This infrastructure setup is designed for both development and production use. Adjust resource limits and configurations based on your specific needs. 