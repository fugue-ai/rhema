# Rhema LGTM Stack

This directory contains the Docker Compose configuration for a minimal LGTM (Loki, Grafana, Tempo, Mimir) observability stack for the Rhema project.

## Components

- **Loki** (port 3100): Log aggregation and storage
- **Grafana** (port 3000): Visualization and dashboards
- **Tempo** (port 3200): Distributed tracing
- **Mimir** (port 9009): Metrics storage (Prometheus-compatible)
- **Promtail** (port 9080): Log collection agent
- **Redis** (port 6379): Caching and session storage

## Quick Start

1. Start the stack:
   ```bash
   docker-compose up -d
   ```

2. Access Grafana:
   - URL: http://localhost:3000
   - Username: `admin`
   - Password: `admin`

3. Access individual services:
   - Mimir (Prometheus API): http://localhost:9009
   - Loki: http://localhost:3100
   - Tempo: http://localhost:3200

## Configuration

All configuration files are located in the `config/` directory:

- `mimir.yaml`: Metrics storage configuration
- `loki.yaml`: Log aggregation configuration
- `tempo.yaml`: Distributed tracing configuration
- `promtail.yaml`: Log collection configuration
- `grafana/`: Grafana provisioning configuration

## Data Persistence

The stack uses Docker volumes for data persistence:
- `mimir_data`: Metrics storage
- `loki_data`: Log storage
- `tempo_data`: Trace storage
- `grafana_data`: Grafana dashboards and settings
- `redis_data`: Redis data

## Integration with Rhema

The Rhema service is configured to send:
- Metrics to Mimir (Prometheus-compatible endpoint)
- Traces to Tempo (OTLP format)
- Logs via Promtail to Loki

## Monitoring

The stack provides comprehensive observability:
- **Metrics**: System and application metrics via Mimir
- **Logs**: Centralized log aggregation via Loki
- **Traces**: Distributed tracing via Tempo
- **Dashboards**: Pre-configured Grafana dashboards

## Scaling

This is a minimal single-node setup. For production use, consider:
- Using external storage backends (S3, GCS, etc.)
- Enabling multi-tenancy
- Adding alerting rules
- Implementing proper authentication and authorization 