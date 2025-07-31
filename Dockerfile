# Multi-stage Dockerfile for Rhema Production Deployment
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libgit2-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release

# Remove dummy main.rs and copy actual source code
RUN rm src/main.rs
COPY src/ ./src/

# Build the application
RUN cargo build --release

# Production runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libgit2-1.1 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -r -s /bin/false rhema

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/rhema /usr/local/bin/rhema

# Create necessary directories
RUN mkdir -p /app/data /app/logs /app/cache && \
    chown -R rhema:rhema /app

# Switch to non-root user
USER rhema

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD rhema health || exit 1

# Expose port (if needed for web interface)
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV RHEMA_DATA_DIR=/app/data
ENV RHEMA_CACHE_DIR=/app/cache
ENV RHEMA_LOG_DIR=/app/logs

# Default command
CMD ["rhema"] 