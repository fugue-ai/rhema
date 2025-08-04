use sqlx::{sqlite::SqlitePool, postgres::PgPool, mysql::MySqlPool, Pool, Database, Row};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::error::{Error, Result};
use crate::types::{DependencyConfig, DependencyType, HealthStatus, ImpactScore, HealthMetrics};
use crate::config::DatabaseConfig;

/// Database connection pool
pub enum DatabasePool {
    Sqlite(SqlitePool),
    Postgres(PgPool),
    MySQL(MySqlPool),
}

/// Storage manager for dependency data
pub struct StorageManager {
    /// Database pool
    pool: DatabasePool,
    /// Database configuration
    config: DatabaseConfig,
}

/// Dependency record for database storage
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DependencyRecord {
    /// Unique identifier
    pub id: String,
    /// Dependency name
    pub name: String,
    /// Dependency description
    pub description: Option<String>,
    /// Dependency type
    pub dependency_type: String,
    /// Target service or resource
    pub target: String,
    /// Operations JSON
    pub operations: String,
    /// Health check configuration JSON
    pub health_check: Option<String>,
    /// Impact configuration JSON
    pub impact_config: Option<String>,
    /// Security requirements JSON
    pub security_requirements: Option<String>,
    /// Performance requirements JSON
    pub performance_requirements: Option<String>,
    /// Metadata JSON
    pub metadata: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Health metrics record for database storage
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct HealthMetricsRecord {
    /// Dependency ID
    pub dependency_id: String,
    /// Response time in milliseconds
    pub response_time_ms: f64,
    /// Availability percentage
    pub availability: f64,
    /// Error rate percentage
    pub error_rate: f64,
    /// Throughput
    pub throughput: f64,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Network latency in milliseconds
    pub network_latency_ms: f64,
    /// Disk usage percentage
    pub disk_usage: f64,
    /// Health status
    pub health_status: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Impact analysis record for database storage
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ImpactAnalysisRecord {
    /// Dependency ID
    pub dependency_id: String,
    /// Business impact score
    pub business_impact: f64,
    /// Revenue impact score
    pub revenue_impact: f64,
    /// User experience impact score
    pub user_experience_impact: f64,
    /// Operational cost impact score
    pub operational_cost_impact: f64,
    /// Security impact score
    pub security_impact: f64,
    /// Compliance impact score
    pub compliance_impact: f64,
    /// Risk level
    pub risk_level: String,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// Dependency relationship record for database storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyRelationshipRecord {
    /// Source dependency ID
    pub source_id: String,
    /// Target dependency ID
    pub target_id: String,
    /// Relationship type
    pub relationship_type: String,
    /// Strength
    pub strength: f64,
    /// Operations JSON
    pub operations: String,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

impl StorageManager {
    /// Create a new storage manager
    pub async fn new(config: DatabaseConfig) -> Result<Self> {
        let pool = match config.database_type {
            crate::config::DatabaseType::Sqlite => {
                let pool = SqlitePool::connect(&config.connection_string).await
                    .map_err(|e| Error::Database(e.into()))?;
                DatabasePool::Sqlite(pool)
            }
            crate::config::DatabaseType::Postgres => {
                let pool = PgPool::connect(&config.connection_string).await
                    .map_err(|e| Error::Database(e.into()))?;
                DatabasePool::Postgres(pool)
            }
            crate::config::DatabaseType::MySQL => {
                let pool = MySqlPool::connect(&config.connection_string).await
                    .map_err(|e| Error::Database(e.into()))?;
                DatabasePool::MySQL(pool)
            }
            crate::config::DatabaseType::InMemory => {
                let pool = SqlitePool::connect("sqlite::memory:").await
                    .map_err(|e| Error::Database(e.into()))?;
                DatabasePool::Sqlite(pool)
            }
        };

        let manager = Self { pool, config };
        manager.initialize_database().await?;
        Ok(manager)
    }

    /// Initialize database tables
    async fn initialize_database(&self) -> Result<()> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS dependencies (
                        id TEXT PRIMARY KEY,
                        name TEXT NOT NULL,
                        description TEXT,
                        dependency_type TEXT NOT NULL,
                        target TEXT NOT NULL,
                        operations TEXT NOT NULL,
                        health_check TEXT,
                        impact_config TEXT,
                        security_requirements TEXT,
                        performance_requirements TEXT,
                        metadata TEXT NOT NULL,
                        created_at DATETIME NOT NULL,
                        updated_at DATETIME NOT NULL
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS health_metrics (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        dependency_id TEXT NOT NULL,
                        response_time_ms REAL NOT NULL,
                        availability REAL NOT NULL,
                        error_rate REAL NOT NULL,
                        throughput REAL NOT NULL,
                        cpu_usage REAL NOT NULL,
                        memory_usage REAL NOT NULL,
                        network_latency_ms REAL NOT NULL,
                        disk_usage REAL NOT NULL,
                        health_status TEXT NOT NULL,
                        timestamp DATETIME NOT NULL,
                        FOREIGN KEY (dependency_id) REFERENCES dependencies (id)
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS impact_analysis (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        dependency_id TEXT NOT NULL,
                        business_impact REAL NOT NULL,
                        revenue_impact REAL NOT NULL,
                        user_experience_impact REAL NOT NULL,
                        operational_cost_impact REAL NOT NULL,
                        security_impact REAL NOT NULL,
                        compliance_impact REAL NOT NULL,
                        risk_level TEXT NOT NULL,
                        timestamp DATETIME NOT NULL,
                        FOREIGN KEY (dependency_id) REFERENCES dependencies (id)
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS dependency_relationships (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        source_id TEXT NOT NULL,
                        target_id TEXT NOT NULL,
                        relationship_type TEXT NOT NULL,
                        strength REAL NOT NULL,
                        operations TEXT NOT NULL,
                        created_at DATETIME NOT NULL,
                        FOREIGN KEY (source_id) REFERENCES dependencies (id),
                        FOREIGN KEY (target_id) REFERENCES dependencies (id)
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS dependencies (
                        id VARCHAR(255) PRIMARY KEY,
                        name VARCHAR(255) NOT NULL,
                        description TEXT,
                        dependency_type VARCHAR(50) NOT NULL,
                        target TEXT NOT NULL,
                        operations JSONB NOT NULL,
                        health_check JSONB,
                        impact_config JSONB,
                        security_requirements JSONB,
                        performance_requirements JSONB,
                        metadata JSONB NOT NULL,
                        created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                        updated_at TIMESTAMP WITH TIME ZONE NOT NULL
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS health_metrics (
                        id SERIAL PRIMARY KEY,
                        dependency_id VARCHAR(255) NOT NULL,
                        response_time_ms DOUBLE PRECISION NOT NULL,
                        availability DOUBLE PRECISION NOT NULL,
                        error_rate DOUBLE PRECISION NOT NULL,
                        throughput DOUBLE PRECISION NOT NULL,
                        cpu_usage DOUBLE PRECISION NOT NULL,
                        memory_usage DOUBLE PRECISION NOT NULL,
                        network_latency_ms DOUBLE PRECISION NOT NULL,
                        disk_usage DOUBLE PRECISION NOT NULL,
                        health_status VARCHAR(50) NOT NULL,
                        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
                        FOREIGN KEY (dependency_id) REFERENCES dependencies (id)
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS impact_analysis (
                        id SERIAL PRIMARY KEY,
                        dependency_id VARCHAR(255) NOT NULL,
                        business_impact DOUBLE PRECISION NOT NULL,
                        revenue_impact DOUBLE PRECISION NOT NULL,
                        user_experience_impact DOUBLE PRECISION NOT NULL,
                        operational_cost_impact DOUBLE PRECISION NOT NULL,
                        security_impact DOUBLE PRECISION NOT NULL,
                        compliance_impact DOUBLE PRECISION NOT NULL,
                        risk_level VARCHAR(50) NOT NULL,
                        timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
                        FOREIGN KEY (dependency_id) REFERENCES dependencies (id)
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS dependency_relationships (
                        id SERIAL PRIMARY KEY,
                        source_id VARCHAR(255) NOT NULL,
                        target_id VARCHAR(255) NOT NULL,
                        relationship_type VARCHAR(100) NOT NULL,
                        strength DOUBLE PRECISION NOT NULL,
                        operations JSONB NOT NULL,
                        created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                        FOREIGN KEY (source_id) REFERENCES dependencies (id),
                        FOREIGN KEY (target_id) REFERENCES dependencies (id)
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS dependencies (
                        id VARCHAR(255) PRIMARY KEY,
                        name VARCHAR(255) NOT NULL,
                        description TEXT,
                        dependency_type VARCHAR(50) NOT NULL,
                        target TEXT NOT NULL,
                        operations JSON NOT NULL,
                        health_check JSON,
                        impact_config JSON,
                        security_requirements JSON,
                        performance_requirements JSON,
                        metadata JSON NOT NULL,
                        created_at TIMESTAMP NOT NULL,
                        updated_at TIMESTAMP NOT NULL
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS health_metrics (
                        id INT AUTO_INCREMENT PRIMARY KEY,
                        dependency_id VARCHAR(255) NOT NULL,
                        response_time_ms DOUBLE NOT NULL,
                        availability DOUBLE NOT NULL,
                        error_rate DOUBLE NOT NULL,
                        throughput DOUBLE NOT NULL,
                        cpu_usage DOUBLE NOT NULL,
                        memory_usage DOUBLE NOT NULL,
                        network_latency_ms DOUBLE NOT NULL,
                        disk_usage DOUBLE NOT NULL,
                        health_status VARCHAR(50) NOT NULL,
                        timestamp TIMESTAMP NOT NULL,
                        FOREIGN KEY (dependency_id) REFERENCES dependencies (id)
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS impact_analysis (
                        id INT AUTO_INCREMENT PRIMARY KEY,
                        dependency_id VARCHAR(255) NOT NULL,
                        business_impact DOUBLE NOT NULL,
                        revenue_impact DOUBLE NOT NULL,
                        user_experience_impact DOUBLE NOT NULL,
                        operational_cost_impact DOUBLE NOT NULL,
                        security_impact DOUBLE NOT NULL,
                        compliance_impact DOUBLE NOT NULL,
                        risk_level VARCHAR(50) NOT NULL,
                        timestamp TIMESTAMP NOT NULL,
                        FOREIGN KEY (dependency_id) REFERENCES dependencies (id)
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;

                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS dependency_relationships (
                        id INT AUTO_INCREMENT PRIMARY KEY,
                        source_id VARCHAR(255) NOT NULL,
                        target_id VARCHAR(255) NOT NULL,
                        relationship_type VARCHAR(100) NOT NULL,
                        strength DOUBLE NOT NULL,
                        operations JSON NOT NULL,
                        created_at TIMESTAMP NOT NULL,
                        FOREIGN KEY (source_id) REFERENCES dependencies (id),
                        FOREIGN KEY (target_id) REFERENCES dependencies (id)
                    )
                    "#,
                )
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
        }

        Ok(())
    }

    /// Save dependency configuration
    pub async fn save_dependency(&self, config: &DependencyConfig) -> Result<()> {
        let operations_json = serde_json::to_string(&config.operations)
            .map_err(|e| Error::Serialization(e.into()))?;

        let health_check_json = config.health_check.as_ref()
            .map(|hc| serde_json::to_string(hc))
            .transpose()
            .map_err(|e| Error::Serialization(e.into()))?;

        let impact_config_json = config.impact_config.as_ref()
            .map(|ic| serde_json::to_string(ic))
            .transpose()
            .map_err(|e| Error::Serialization(e.into()))?;

        let security_requirements_json = config.security_requirements.as_ref()
            .map(|sr| serde_json::to_string(sr))
            .transpose()
            .map_err(|e| Error::Serialization(e.into()))?;

        let performance_requirements_json = config.performance_requirements.as_ref()
            .map(|pr| serde_json::to_string(pr))
            .transpose()
            .map_err(|e| Error::Serialization(e.into()))?;

        let metadata_json = serde_json::to_string(&config.metadata)
            .map_err(|e| Error::Serialization(e.into()))?;

        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    INSERT OR REPLACE INTO dependencies 
                    (id, name, description, dependency_type, target, operations, 
                     health_check, impact_config, security_requirements, 
                     performance_requirements, metadata, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(&config.id)
                .bind(&config.name)
                .bind(&config.description)
                .bind(&config.dependency_type.to_string())
                .bind(&config.target)
                .bind(&operations_json)
                .bind(&health_check_json)
                .bind(&impact_config_json)
                .bind(&security_requirements_json)
                .bind(&performance_requirements_json)
                .bind(&metadata_json)
                .bind(config.created_at)
                .bind(config.updated_at)
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO dependencies 
                    (id, name, description, dependency_type, target, operations, 
                     health_check, impact_config, security_requirements, 
                     performance_requirements, metadata, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
                    ON CONFLICT (id) DO UPDATE SET
                    name = EXCLUDED.name,
                    description = EXCLUDED.description,
                    dependency_type = EXCLUDED.dependency_type,
                    target = EXCLUDED.target,
                    operations = EXCLUDED.operations,
                    health_check = EXCLUDED.health_check,
                    impact_config = EXCLUDED.impact_config,
                    security_requirements = EXCLUDED.security_requirements,
                    performance_requirements = EXCLUDED.performance_requirements,
                    metadata = EXCLUDED.metadata,
                    updated_at = EXCLUDED.updated_at
                    "#,
                )
                .bind(&config.id)
                .bind(&config.name)
                .bind(&config.description)
                .bind(&config.dependency_type.to_string())
                .bind(&config.target)
                .bind(&operations_json)
                .bind(&health_check_json)
                .bind(&impact_config_json)
                .bind(&security_requirements_json)
                .bind(&performance_requirements_json)
                .bind(&metadata_json)
                .bind(config.created_at)
                .bind(config.updated_at)
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO dependencies 
                    (id, name, description, dependency_type, target, operations, 
                     health_check, impact_config, security_requirements, 
                     performance_requirements, metadata, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    ON DUPLICATE KEY UPDATE
                    name = VALUES(name),
                    description = VALUES(description),
                    dependency_type = VALUES(dependency_type),
                    target = VALUES(target),
                    operations = VALUES(operations),
                    health_check = VALUES(health_check),
                    impact_config = VALUES(impact_config),
                    security_requirements = VALUES(security_requirements),
                    performance_requirements = VALUES(performance_requirements),
                    metadata = VALUES(metadata),
                    updated_at = VALUES(updated_at)
                    "#,
                )
                .bind(&config.id)
                .bind(&config.name)
                .bind(&config.description)
                .bind(&config.dependency_type.to_string())
                .bind(&config.target)
                .bind(&operations_json)
                .bind(&health_check_json)
                .bind(&impact_config_json)
                .bind(&security_requirements_json)
                .bind(&performance_requirements_json)
                .bind(&metadata_json)
                .bind(config.created_at)
                .bind(config.updated_at)
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
        }

        Ok(())
    }

    /// Load dependency configuration
    pub async fn load_dependency(&self, id: &str) -> Result<Option<DependencyConfig>> {
        let record = match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query_as::<_, DependencyRecord>(
                    "SELECT * FROM dependencies WHERE id = ?"
                )
                .bind(id)
                .fetch_optional(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query_as::<_, DependencyRecord>(
                    "SELECT * FROM dependencies WHERE id = $1"
                )
                .bind(id)
                .fetch_optional(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query_as::<_, DependencyRecord>(
                    "SELECT * FROM dependencies WHERE id = ?"
                )
                .bind(id)
                .fetch_optional(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
        };

        if let Some(record) = record {
            let config = self.record_to_config(record)?;
            Ok(Some(config))
        } else {
            Ok(None)
        }
    }

    /// Load all dependency configurations
    pub async fn load_all_dependencies(&self) -> Result<Vec<DependencyConfig>> {
        let records = match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query_as::<_, DependencyRecord>(
                    "SELECT * FROM dependencies ORDER BY created_at"
                )
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query_as::<_, DependencyRecord>(
                    "SELECT * FROM dependencies ORDER BY created_at"
                )
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query_as::<_, DependencyRecord>(
                    "SELECT * FROM dependencies ORDER BY created_at"
                )
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
        };

        let mut configs = Vec::new();
        for record in records {
            let config = self.record_to_config(record)?;
            configs.push(config);
        }

        Ok(configs)
    }

    /// Delete dependency configuration
    pub async fn delete_dependency(&self, id: &str) -> Result<()> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query("DELETE FROM dependencies WHERE id = ?")
                    .bind(id)
                    .execute(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query("DELETE FROM dependencies WHERE id = $1")
                    .bind(id)
                    .execute(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query("DELETE FROM dependencies WHERE id = ?")
                    .bind(id)
                    .execute(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;
            }
        }

        Ok(())
    }

    /// Save health metrics
    pub async fn save_health_metrics(&self, dependency_id: &str, metrics: &HealthMetrics) -> Result<()> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO health_metrics 
                    (dependency_id, response_time_ms, availability, error_rate, throughput,
                     cpu_usage, memory_usage, network_latency_ms, disk_usage, health_status, timestamp)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(dependency_id)
                .bind(metrics.response_time_ms)
                .bind(metrics.availability)
                .bind(metrics.error_rate)
                .bind(metrics.throughput)
                .bind(metrics.cpu_usage)
                .bind(metrics.memory_usage)
                .bind(metrics.network_latency_ms)
                .bind(metrics.disk_usage)
                .bind(&HealthStatus::from(metrics.health_score()).to_string())
                .bind(metrics.timestamp)
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO health_metrics 
                    (dependency_id, response_time_ms, availability, error_rate, throughput,
                     cpu_usage, memory_usage, network_latency_ms, disk_usage, health_status, timestamp)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                    "#,
                )
                .bind(dependency_id)
                .bind(metrics.response_time_ms)
                .bind(metrics.availability)
                .bind(metrics.error_rate)
                .bind(metrics.throughput)
                .bind(metrics.cpu_usage)
                .bind(metrics.memory_usage)
                .bind(metrics.network_latency_ms)
                .bind(metrics.disk_usage)
                .bind(&HealthStatus::from(metrics.health_score()).to_string())
                .bind(metrics.timestamp)
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO health_metrics 
                    (dependency_id, response_time_ms, availability, error_rate, throughput,
                     cpu_usage, memory_usage, network_latency_ms, disk_usage, health_status, timestamp)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(dependency_id)
                .bind(metrics.response_time_ms)
                .bind(metrics.availability)
                .bind(metrics.error_rate)
                .bind(metrics.throughput)
                .bind(metrics.cpu_usage)
                .bind(metrics.memory_usage)
                .bind(metrics.network_latency_ms)
                .bind(metrics.disk_usage)
                .bind(&HealthStatus::from(metrics.health_score()).to_string())
                .bind(metrics.timestamp)
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
        }

        Ok(())
    }

    /// Load health metrics for a dependency
    pub async fn load_health_metrics(&self, dependency_id: &str, limit: Option<i64>) -> Result<Vec<HealthMetrics>> {
        let limit = limit.unwrap_or(100);
        
        let records = match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query_as::<_, HealthMetricsRecord>(
                    "SELECT * FROM health_metrics WHERE dependency_id = ? ORDER BY timestamp DESC LIMIT ?"
                )
                .bind(dependency_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query_as::<_, HealthMetricsRecord>(
                    "SELECT * FROM health_metrics WHERE dependency_id = $1 ORDER BY timestamp DESC LIMIT $2"
                )
                .bind(dependency_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query_as::<_, HealthMetricsRecord>(
                    "SELECT * FROM health_metrics WHERE dependency_id = ? ORDER BY timestamp DESC LIMIT ?"
                )
                .bind(dependency_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
        };

        let mut metrics = Vec::new();
        for record in records {
            let metric = HealthMetrics::new(
                record.response_time_ms,
                record.availability,
                record.error_rate,
                record.throughput,
                record.cpu_usage,
                record.memory_usage,
                record.network_latency_ms,
                record.disk_usage,
            )?;
            metrics.push(metric);
        }

        Ok(metrics)
    }

    /// Save impact analysis
    pub async fn save_impact_analysis(&self, dependency_id: &str, impact_score: &ImpactScore) -> Result<()> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO impact_analysis 
                    (dependency_id, business_impact, revenue_impact, user_experience_impact,
                     operational_cost_impact, security_impact, compliance_impact, risk_level, timestamp)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(dependency_id)
                .bind(impact_score.business_impact)
                .bind(impact_score.revenue_impact)
                .bind(impact_score.user_experience_impact)
                .bind(impact_score.operational_cost_impact)
                .bind(impact_score.security_impact)
                .bind(impact_score.compliance_impact)
                .bind(&impact_score.risk_level.to_string())
                .bind(impact_score.timestamp)
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO impact_analysis 
                    (dependency_id, business_impact, revenue_impact, user_experience_impact,
                     operational_cost_impact, security_impact, compliance_impact, risk_level, timestamp)
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                    "#,
                )
                .bind(dependency_id)
                .bind(impact_score.business_impact)
                .bind(impact_score.revenue_impact)
                .bind(impact_score.user_experience_impact)
                .bind(impact_score.operational_cost_impact)
                .bind(impact_score.security_impact)
                .bind(impact_score.compliance_impact)
                .bind(&impact_score.risk_level.to_string())
                .bind(impact_score.timestamp)
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO impact_analysis 
                    (dependency_id, business_impact, revenue_impact, user_experience_impact,
                     operational_cost_impact, security_impact, compliance_impact, risk_level, timestamp)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                )
                .bind(dependency_id)
                .bind(impact_score.business_impact)
                .bind(impact_score.revenue_impact)
                .bind(impact_score.user_experience_impact)
                .bind(impact_score.operational_cost_impact)
                .bind(impact_score.security_impact)
                .bind(impact_score.compliance_impact)
                .bind(&impact_score.risk_level.to_string())
                .bind(impact_score.timestamp)
                .execute(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?;
            }
        }

        Ok(())
    }

    /// Load impact analysis for a dependency
    pub async fn load_impact_analysis(&self, dependency_id: &str, limit: Option<i64>) -> Result<Vec<ImpactScore>> {
        let limit = limit.unwrap_or(100);
        
        let records = match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query_as::<_, ImpactAnalysisRecord>(
                    "SELECT * FROM impact_analysis WHERE dependency_id = ? ORDER BY timestamp DESC LIMIT ?"
                )
                .bind(dependency_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query_as::<_, ImpactAnalysisRecord>(
                    "SELECT * FROM impact_analysis WHERE dependency_id = $1 ORDER BY timestamp DESC LIMIT $2"
                )
                .bind(dependency_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
            DatabasePool::MySQL(pool) => {
                sqlx::query_as::<_, ImpactAnalysisRecord>(
                    "SELECT * FROM impact_analysis WHERE dependency_id = ? ORDER BY timestamp DESC LIMIT ?"
                )
                .bind(dependency_id)
                .bind(limit)
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
            }
        };

        let mut impact_scores = Vec::new();
        for record in records {
            let impact_score = ImpactScore::new(
                record.business_impact,
                record.revenue_impact,
                record.user_experience_impact,
                record.operational_cost_impact,
                record.security_impact,
                record.compliance_impact,
            )?;
            impact_scores.push(impact_score);
        }

        Ok(impact_scores)
    }

    /// Convert database record to dependency configuration
    fn record_to_config(&self, record: DependencyRecord) -> Result<DependencyConfig> {
        let operations: Vec<String> = serde_json::from_str(&record.operations)
            .map_err(|e| Error::Serialization(e.into()))?;

        let health_check = if let Some(hc_json) = record.health_check {
            Some(serde_json::from_str(&hc_json)
                .map_err(|e| Error::Serialization(e.into()))?)
        } else {
            None
        };

        let impact_config = if let Some(ic_json) = record.impact_config {
            Some(serde_json::from_str(&ic_json)
                .map_err(|e| Error::Serialization(e.into()))?)
        } else {
            None
        };

        let security_requirements = if let Some(sr_json) = record.security_requirements {
            Some(serde_json::from_str(&sr_json)
                .map_err(|e| Error::Serialization(e.into()))?)
        } else {
            None
        };

        let performance_requirements = if let Some(pr_json) = record.performance_requirements {
            Some(serde_json::from_str(&pr_json)
                .map_err(|e| Error::Serialization(e.into()))?)
        } else {
            None
        };

        let metadata: HashMap<String, String> = serde_json::from_str(&record.metadata)
            .map_err(|e| Error::Serialization(e.into()))?;

        let dependency_type = match record.dependency_type.as_str() {
            "DataFlow" => DependencyType::DataFlow,
            "ApiCall" => DependencyType::ApiCall,
            "Infrastructure" => DependencyType::Infrastructure,
            "BusinessLogic" => DependencyType::BusinessLogic,
            "Security" => DependencyType::Security,
            "Monitoring" => DependencyType::Monitoring,
            "Configuration" => DependencyType::Configuration,
            "Deployment" => DependencyType::Deployment,
            _ => return Err(Error::InvalidDependencyType(record.dependency_type)),
        };

        Ok(DependencyConfig {
            id: record.id,
            name: record.name,
            description: record.description,
            dependency_type,
            target: record.target,
            operations,
            health_check,
            impact_config,
            security_requirements,
            performance_requirements,
            metadata,
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }

    /// Get storage statistics
    pub async fn get_statistics(&self) -> Result<StorageStatistics> {
        let (dependency_count, health_metrics_count, impact_analysis_count) = match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let dependency_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM dependencies")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;

                let health_metrics_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM health_metrics")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;

                let impact_analysis_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM impact_analysis")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;

                (dependency_count, health_metrics_count, impact_analysis_count)
            }
            DatabasePool::Postgres(pool) => {
                let dependency_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM dependencies")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;

                let health_metrics_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM health_metrics")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;

                let impact_analysis_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM impact_analysis")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;

                (dependency_count, health_metrics_count, impact_analysis_count)
            }
            DatabasePool::MySQL(pool) => {
                let dependency_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM dependencies")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;

                let health_metrics_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM health_metrics")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;

                let impact_analysis_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM impact_analysis")
                    .fetch_one(pool)
                    .await
                    .map_err(|e| Error::Database(e.into()))?;

                (dependency_count, health_metrics_count, impact_analysis_count)
            }
        };

        Ok(StorageStatistics {
            dependency_count: dependency_count as usize,
            health_metrics_count: health_metrics_count as usize,
            impact_analysis_count: impact_analysis_count as usize,
            database_type: self.config.database_type.to_string(),
            last_updated: Utc::now(),
        })
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStatistics {
    /// Number of dependencies
    pub dependency_count: usize,
    /// Number of health metrics records
    pub health_metrics_count: usize,
    /// Number of impact analysis records
    pub impact_analysis_count: usize,
    /// Database type
    pub database_type: String,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencyConfig;

    #[tokio::test]
    async fn test_storage_manager_new() {
        let config = DatabaseConfig {
            database_type: crate::config::DatabaseType::InMemory,
            connection_string: "sqlite::memory:".to_string(),
            pool_size: 5,
            connection_timeout: 30,
            query_timeout: 60,
            enable_migrations: true,
            migration_directory: None,
        };

        let manager = StorageManager::new(config).await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_save_and_load_dependency() {
        let config = DatabaseConfig {
            database_type: crate::config::DatabaseType::InMemory,
            connection_string: "sqlite::memory:".to_string(),
            pool_size: 5,
            connection_timeout: 30,
            query_timeout: 60,
            enable_migrations: true,
            migration_directory: None,
        };

        let manager = StorageManager::new(config).await.unwrap();

        let dependency_config = DependencyConfig::new(
            "test-1".to_string(),
            "Test Dependency".to_string(),
            DependencyType::ApiCall,
            "http://test.example.com".to_string(),
            vec!["GET".to_string()],
        ).unwrap();

        // Save dependency
        let save_result = manager.save_dependency(&dependency_config).await;
        assert!(save_result.is_ok());

        // Load dependency
        let loaded_config = manager.load_dependency("test-1").await.unwrap();
        assert!(loaded_config.is_some());
        assert_eq!(loaded_config.unwrap().id, "test-1");
    }

    #[test]
    fn test_storage_statistics() {
        let stats = StorageStatistics {
            dependency_count: 10,
            health_metrics_count: 100,
            impact_analysis_count: 50,
            database_type: "sqlite".to_string(),
            last_updated: Utc::now(),
        };

        assert_eq!(stats.dependency_count, 10);
        assert_eq!(stats.health_metrics_count, 100);
        assert_eq!(stats.impact_analysis_count, 50);
        assert_eq!(stats.database_type, "sqlite");
    }
} 