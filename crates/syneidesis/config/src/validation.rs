/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Configuration validation

use crate::error::{ConfigError, ValidationError};
use crate::types::SyneidesisConfig;
use std::collections::HashMap;
use tracing::{debug, error, warn};

/// Configuration validator
pub struct ConfigValidator {
    /// Validation rules
    rules: Vec<ValidationRule>,

    /// Custom validation functions
    custom_validators: HashMap<
        String,
        Box<dyn Fn(&SyneidesisConfig) -> Result<(), ValidationError> + Send + Sync>,
    >,
}

impl ConfigValidator {
    /// Create a new configuration validator
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            custom_validators: HashMap::new(),
        }
    }

    /// Add a validation rule
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.push(rule);
    }

    /// Add a custom validator
    pub fn add_custom_validator<F>(&mut self, name: &str, validator: F)
    where
        F: Fn(&SyneidesisConfig) -> Result<(), ValidationError> + Send + Sync + 'static,
    {
        self.custom_validators
            .insert(name.to_string(), Box::new(validator));
    }

    /// Validate configuration
    pub fn validate(&self, config: &SyneidesisConfig) -> Result<(), ConfigError> {
        debug!("Validating configuration");

        let mut errors = Vec::new();

        // Run built-in validation rules
        for rule in &self.rules {
            if let Err(error) = rule.validate(config) {
                errors.push(error);
            }
        }

        // Run custom validators
        for (name, validator) in &self.custom_validators {
            if let Err(error) = validator(config) {
                error!("Custom validator '{}' failed: {}", name, error);
                errors.push(error);
            }
        }

        // Run field-specific validations
        if let Err(error) = self.validate_system_config(config) {
            errors.push(error);
        }

        if let Err(error) = self.validate_agent_config(config) {
            errors.push(error);
        }

        if let Err(error) = self.validate_coordination_config(config) {
            errors.push(error);
        }

        if let Err(error) = self.validate_grpc_config(config) {
            errors.push(error);
        }

        if let Err(error) = self.validate_http_config(config) {
            errors.push(error);
        }

        if let Err(error) = self.validate_network_config(config) {
            errors.push(error);
        }

        if let Err(error) = self.validate_security_config(config) {
            errors.push(error);
        }

        if let Err(error) = self.validate_logging_config(config) {
            errors.push(error);
        }

        if let Err(error) = self.validate_validation_config(config) {
            errors.push(error);
        }

        // Run cross-field validations
        if let Err(error) = self.validate_cross_field_rules(config) {
            errors.push(error);
        }

        if errors.is_empty() {
            debug!("Configuration validation passed");
            Ok(())
        } else {
            error!(
                "Configuration validation failed with {} errors",
                errors.len()
            );
            Err(ConfigError::ValidationError {
                message: format!(
                    "Validation failed with {} errors: {:?}",
                    errors.len(),
                    errors
                ),
            })
        }
    }

    /// Validate system configuration
    fn validate_system_config(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        if let Some(system) = &config.system {
            // Validate system name
            if system.name.is_empty() {
                return Err(ValidationError::RequiredFieldMissing {
                    field: "system.name".to_string(),
                });
            }

            if system.name.len() > 100 {
                return Err(ValidationError::LengthValidation {
                    field: "system.name".to_string(),
                    length: system.name.len(),
                    min: 1,
                    max: 100,
                });
            }

            // Validate system version
            if system.version.is_empty() {
                return Err(ValidationError::RequiredFieldMissing {
                    field: "system.version".to_string(),
                });
            }

            if system.version.len() > 50 {
                return Err(ValidationError::LengthValidation {
                    field: "system.version".to_string(),
                    length: system.version.len(),
                    min: 1,
                    max: 50,
                });
            }

            // Validate environment
            let valid_environments = vec![
                "development".to_string(),
                "staging".to_string(),
                "production".to_string(),
                "test".to_string(),
            ];
            if !valid_environments.contains(&system.environment) {
                return Err(ValidationError::InvalidEnumValue {
                    field: "system.environment".to_string(),
                    value: system.environment.clone(),
                    allowed: valid_environments,
                });
            }

            // Validate max concurrent operations
            if system.max_concurrent_operations == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "system.max_concurrent_operations".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "10000".to_string(),
                });
            }

            if system.max_concurrent_operations > 10000 {
                return Err(ValidationError::RangeValidation {
                    field: "system.max_concurrent_operations".to_string(),
                    value: system.max_concurrent_operations.to_string(),
                    min: "1".to_string(),
                    max: "10000".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate agent configuration
    fn validate_agent_config(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        if let Some(agent) = &config.agent {
            // Validate max agents
            if agent.max_agents == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "agent.max_agents".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "10000".to_string(),
                });
            }

            if agent.max_agents > 10000 {
                return Err(ValidationError::RangeValidation {
                    field: "agent.max_agents".to_string(),
                    value: agent.max_agents.to_string(),
                    min: "1".to_string(),
                    max: "10000".to_string(),
                });
            }

            // Validate heartbeat interval
            if agent.heartbeat_interval.as_secs() == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "agent.heartbeat_interval".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "3600".to_string(),
                });
            }

            if agent.heartbeat_interval.as_secs() > 3600 {
                return Err(ValidationError::RangeValidation {
                    field: "agent.heartbeat_interval".to_string(),
                    value: agent.heartbeat_interval.as_secs().to_string(),
                    min: "1".to_string(),
                    max: "3600".to_string(),
                });
            }

            // Validate resource limits if present
            if let Some(limits) = &agent.resource_limits {
                if limits.max_cpu_usage <= 0.0 || limits.max_cpu_usage > 100.0 {
                    return Err(ValidationError::RangeValidation {
                        field: "agent.resource_limits.max_cpu_usage".to_string(),
                        value: limits.max_cpu_usage.to_string(),
                        min: "1.0".to_string(),
                        max: "100.0".to_string(),
                    });
                }

                if limits.max_memory_usage == 0 {
                    return Err(ValidationError::RangeValidation {
                        field: "agent.resource_limits.max_memory_usage".to_string(),
                        value: "0".to_string(),
                        min: "1".to_string(),
                        max: "1099511627776".to_string(), // 1TB
                    });
                }
            }
        }

        Ok(())
    }

    /// Validate coordination configuration
    fn validate_coordination_config(
        &self,
        config: &SyneidesisConfig,
    ) -> Result<(), ValidationError> {
        if let Some(coordination) = &config.coordination {
            // Validate max agents
            if coordination.max_agents == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "coordination.max_agents".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "10000".to_string(),
                });
            }

            // Validate heartbeat interval
            if coordination.heartbeat_interval == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "coordination.heartbeat_interval".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "3600".to_string(),
                });
            }

            // Validate conflict resolution strategy
            let valid_strategies = vec![
                "priority".to_string(),
                "timestamp".to_string(),
                "random".to_string(),
                "round_robin".to_string(),
            ];
            if !valid_strategies.contains(&coordination.conflict_resolution_strategy) {
                return Err(ValidationError::InvalidEnumValue {
                    field: "coordination.conflict_resolution_strategy".to_string(),
                    value: coordination.conflict_resolution_strategy.clone(),
                    allowed: valid_strategies,
                });
            }
        }

        Ok(())
    }

    /// Validate gRPC configuration
    fn validate_grpc_config(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        if let Some(grpc) = &config.grpc {
            // Validate address format
            if grpc.addr.is_empty() {
                return Err(ValidationError::RequiredFieldMissing {
                    field: "grpc.addr".to_string(),
                });
            }

            // Basic address format validation (host:port)
            if !grpc.addr.contains(':') {
                return Err(ValidationError::FormatValidation {
                    field: "grpc.addr".to_string(),
                    message: "Address must be in format 'host:port'".to_string(),
                });
            }

            // Validate max message size
            if grpc.max_message_size == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "grpc.max_message_size".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "1073741824".to_string(), // 1GB
                });
            }

            // Validate connection timeout
            if grpc.connection_timeout == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "grpc.connection_timeout".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "3600".to_string(),
                });
            }

            // Validate TLS configuration if present
            if let Some(tls) = &grpc.tls {
                if tls.cert_file.is_empty() {
                    return Err(ValidationError::RequiredFieldMissing {
                        field: "grpc.tls.cert_file".to_string(),
                    });
                }

                if tls.key_file.is_empty() {
                    return Err(ValidationError::RequiredFieldMissing {
                        field: "grpc.tls.key_file".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Validate HTTP configuration
    fn validate_http_config(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        if let Some(http) = &config.http {
            // Validate address
            if http.addr.is_empty() {
                return Err(ValidationError::RequiredFieldMissing {
                    field: "http.addr".to_string(),
                });
            }

            // Validate port
            if http.port == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "http.port".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "65535".to_string(),
                });
            }

            // Validate max request size
            if http.max_request_size == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "http.max_request_size".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "1073741824".to_string(), // 1GB
                });
            }

            // Validate request timeout
            if http.request_timeout == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "http.request_timeout".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "3600".to_string(),
                });
            }

            // Validate rate limit
            if http.enable_rate_limiting && http.rate_limit == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "http.rate_limit".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "1000000".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate network configuration
    fn validate_network_config(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        if let Some(network) = &config.network {
            // Validate bind address
            if network.bind_addr.is_empty() {
                return Err(ValidationError::RequiredFieldMissing {
                    field: "network.bind_addr".to_string(),
                });
            }

            // Validate socket buffer size
            if network.socket_buffer_size == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "network.socket_buffer_size".to_string(),
                    value: "0".to_string(),
                    min: "1024".to_string(),
                    max: "67108864".to_string(), // 64MB
                });
            }

            // Validate multicast TTL
            if network.enable_multicast && network.multicast_ttl == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "network.multicast_ttl".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "255".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate security configuration
    fn validate_security_config(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        if let Some(security) = &config.security {
            // Validate JWT expiration
            if security.enable_auth && security.jwt_expiration == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "security.jwt_expiration".to_string(),
                    value: "0".to_string(),
                    min: "60".to_string(),
                    max: "86400".to_string(),
                });
            }

            // Validate rate limit window
            if security.enable_rate_limiting && security.rate_limit_window == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "security.rate_limit_window".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "3600".to_string(),
                });
            }

            // Validate rate limit max requests
            if security.enable_rate_limiting && security.rate_limit_max_requests == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "security.rate_limit_max_requests".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "10000".to_string(),
                });
            }

            // Validate session timeout
            if security.enable_session_management && security.session_timeout == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "security.session_timeout".to_string(),
                    value: "0".to_string(),
                    min: "60".to_string(),
                    max: "86400".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate logging configuration
    fn validate_logging_config(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        if let Some(logging) = &config.logging {
            // Validate log level
            let valid_levels = vec![
                "trace".to_string(),
                "debug".to_string(),
                "info".to_string(),
                "warn".to_string(),
                "error".to_string(),
            ];
            if !valid_levels.contains(&logging.level.to_lowercase()) {
                return Err(ValidationError::InvalidEnumValue {
                    field: "logging.level".to_string(),
                    value: logging.level.clone(),
                    allowed: valid_levels,
                });
            }

            // Validate log format
            let valid_formats = vec![
                "text".to_string(),
                "json".to_string(),
                "structured".to_string(),
            ];
            if !valid_formats.contains(&logging.format.to_lowercase()) {
                return Err(ValidationError::InvalidEnumValue {
                    field: "logging.format".to_string(),
                    value: logging.format.clone(),
                    allowed: valid_formats,
                });
            }

            // Validate rotation size
            if logging.enable_rotation && logging.rotation_size == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "logging.rotation_size".to_string(),
                    value: "0".to_string(),
                    min: "1024".to_string(),
                    max: "1073741824".to_string(), // 1GB
                });
            }

            // Validate rotation count
            if logging.enable_rotation && logging.rotation_count == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "logging.rotation_count".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "1000".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate validation configuration
    fn validate_validation_config(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        if let Some(validation) = &config.validation {
            // Validate validation mode
            let valid_modes = vec![
                "strict".to_string(),
                "lenient".to_string(),
                "warn".to_string(),
            ];
            if !valid_modes.contains(&validation.mode.to_lowercase()) {
                return Err(ValidationError::InvalidEnumValue {
                    field: "validation.mode".to_string(),
                    value: validation.mode.clone(),
                    allowed: valid_modes,
                });
            }

            // Validate error mode
            let valid_error_modes = vec![
                "collect".to_string(),
                "fail_fast".to_string(),
                "warn".to_string(),
            ];
            if !valid_error_modes.contains(&validation.error_mode.to_lowercase()) {
                return Err(ValidationError::InvalidEnumValue {
                    field: "validation.error_mode".to_string(),
                    value: validation.error_mode.clone(),
                    allowed: valid_error_modes,
                });
            }

            // Validate max errors
            if validation.max_errors == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "validation.max_errors".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "10000".to_string(),
                });
            }

            // Validate cache size
            if validation.enable_caching && validation.cache_size == 0 {
                return Err(ValidationError::RangeValidation {
                    field: "validation.cache_size".to_string(),
                    value: "0".to_string(),
                    min: "1".to_string(),
                    max: "100000".to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate cross-field rules
    fn validate_cross_field_rules(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        // Example cross-field validation: if gRPC is enabled, coordination must also be enabled
        if config.grpc.is_some() && config.coordination.is_none() {
            return Err(ValidationError::CrossFieldValidation {
                message: "gRPC configuration requires coordination configuration".to_string(),
            });
        }

        // Example: if HTTP rate limiting is enabled, security rate limiting should also be enabled
        if let Some(http) = &config.http {
            if let Some(security) = &config.security {
                if http.enable_rate_limiting && !security.enable_rate_limiting {
                    warn!("HTTP rate limiting is enabled but security rate limiting is disabled");
                }
            }
        }

        // Example: validate that agent max_agents doesn't exceed coordination max_agents
        if let Some(agent) = &config.agent {
            if let Some(coordination) = &config.coordination {
                if agent.max_agents > coordination.max_agents {
                    return Err(ValidationError::CrossFieldValidation {
                        message: format!(
                            "Agent max_agents ({}) cannot exceed coordination max_agents ({})",
                            agent.max_agents, coordination.max_agents
                        ),
                    });
                }
            }
        }

        Ok(())
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation rule
pub struct ValidationRule {
    /// Rule name
    name: String,

    /// Rule description
    description: String,

    /// Validation function
    validator: Box<dyn Fn(&SyneidesisConfig) -> Result<(), ValidationError> + Send + Sync>,
}

impl ValidationRule {
    /// Create a new validation rule
    pub fn new<F>(name: &str, description: &str, validator: F) -> Self
    where
        F: Fn(&SyneidesisConfig) -> Result<(), ValidationError> + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            validator: Box::new(validator),
        }
    }

    /// Validate using this rule
    pub fn validate(&self, config: &SyneidesisConfig) -> Result<(), ValidationError> {
        (self.validator)(config)
    }

    /// Get rule name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get rule description
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// Built-in validation rules
pub mod rules {
    use super::*;

    /// Rule: System configuration must be present
    pub fn system_config_required() -> ValidationRule {
        ValidationRule::new(
            "system_config_required",
            "System configuration must be present",
            |config| {
                if config.system.is_none() {
                    Err(ValidationError::RequiredFieldMissing {
                        field: "system".to_string(),
                    })
                } else {
                    Ok(())
                }
            },
        )
    }

    /// Rule: Agent configuration must be present
    pub fn agent_config_required() -> ValidationRule {
        ValidationRule::new(
            "agent_config_required",
            "Agent configuration must be present",
            |config| {
                if config.agent.is_none() {
                    Err(ValidationError::RequiredFieldMissing {
                        field: "agent".to_string(),
                    })
                } else {
                    Ok(())
                }
            },
        )
    }

    /// Rule: gRPC configuration must be present
    pub fn grpc_config_required() -> ValidationRule {
        ValidationRule::new(
            "grpc_config_required",
            "gRPC configuration must be present",
            |config| {
                if config.grpc.is_none() {
                    Err(ValidationError::RequiredFieldMissing {
                        field: "grpc".to_string(),
                    })
                } else {
                    Ok(())
                }
            },
        )
    }

    /// Rule: HTTP configuration must be present
    pub fn http_config_required() -> ValidationRule {
        ValidationRule::new(
            "http_config_required",
            "HTTP configuration must be present",
            |config| {
                if config.http.is_none() {
                    Err(ValidationError::RequiredFieldMissing {
                        field: "http".to_string(),
                    })
                } else {
                    Ok(())
                }
            },
        )
    }

    /// Rule: Logging configuration must be present
    pub fn logging_config_required() -> ValidationRule {
        ValidationRule::new(
            "logging_config_required",
            "Logging configuration must be present",
            |config| {
                if config.logging.is_none() {
                    Err(ValidationError::RequiredFieldMissing {
                        field: "logging".to_string(),
                    })
                } else {
                    Ok(())
                }
            },
        )
    }

    /// Rule: Production environment requires security configuration
    pub fn production_security_required() -> ValidationRule {
        ValidationRule::new(
            "production_security_required",
            "Production environment requires security configuration",
            |config| {
                if let Some(system) = &config.system {
                    if system.environment == "production" && config.security.is_none() {
                        return Err(ValidationError::CrossFieldValidation {
                            message: "Production environment requires security configuration"
                                .to_string(),
                        });
                    }
                }
                Ok(())
            },
        )
    }

    /// Rule: TLS must be enabled for production gRPC
    pub fn production_grpc_tls_required() -> ValidationRule {
        ValidationRule::new(
            "production_grpc_tls_required",
            "Production environment requires TLS for gRPC",
            |config| {
                if let Some(system) = &config.system {
                    if system.environment == "production" {
                        if let Some(grpc) = &config.grpc {
                            if grpc.tls.is_none() {
                                return Err(ValidationError::CrossFieldValidation {
                                    message: "Production environment requires TLS for gRPC"
                                        .to_string(),
                                });
                            }
                        }
                    }
                }
                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SyneidesisConfig;

    #[test]
    fn test_config_validator_new() {
        let validator = ConfigValidator::new();
        assert!(validator.rules.is_empty());
        assert!(validator.custom_validators.is_empty());
    }

    #[test]
    fn test_validation_rule_new() {
        let rule = ValidationRule::new("test_rule", "Test validation rule", |_config| Ok(()));

        assert_eq!(rule.name(), "test_rule");
        assert_eq!(rule.description(), "Test validation rule");
    }

    #[test]
    fn test_system_config_validation() {
        let validator = ConfigValidator::new();
        let mut config = SyneidesisConfig::default();

        // Test with valid system config
        assert!(validator.validate(&config).is_ok());

        // Test with invalid system name
        if let Some(system) = &mut config.system {
            system.name = "".to_string();
        }
        assert!(validator.validate(&config).is_err());

        // Test with invalid environment
        if let Some(system) = &mut config.system {
            system.name = "test".to_string();
            system.environment = "invalid".to_string();
        }
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_agent_config_validation() {
        let validator = ConfigValidator::new();
        let mut config = SyneidesisConfig::default();

        // Test with valid agent config
        assert!(validator.validate(&config).is_ok());

        // Test with invalid max agents
        if let Some(agent) = &mut config.agent {
            agent.max_agents = 0;
        }
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_grpc_config_validation() {
        let validator = ConfigValidator::new();
        let mut config = SyneidesisConfig::default();

        // Test with valid gRPC config
        assert!(validator.validate(&config).is_ok());

        // Test with invalid address
        if let Some(grpc) = &mut config.grpc {
            grpc.addr = "".to_string();
        }
        assert!(validator.validate(&config).is_err());

        // Test with invalid address format
        if let Some(grpc) = &mut config.grpc {
            grpc.addr = "invalid-address".to_string();
        }
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_http_config_validation() {
        let validator = ConfigValidator::new();
        let mut config = SyneidesisConfig::default();

        // Test with valid HTTP config
        assert!(validator.validate(&config).is_ok());

        // Test with invalid port
        if let Some(http) = &mut config.http {
            http.port = 0;
        }
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_cross_field_validation() {
        let validator = ConfigValidator::new();
        let mut config = SyneidesisConfig::default();

        // Test with valid cross-field config
        assert!(validator.validate(&config).is_ok());

        // Test with gRPC but no coordination
        config.coordination = None;
        assert!(validator.validate(&config).is_err());
    }

    #[test]
    fn test_built_in_rules() {
        let rule = rules::system_config_required();
        assert_eq!(rule.name(), "system_config_required");
        assert_eq!(rule.description(), "System configuration must be present");

        let config = SyneidesisConfig::default();
        assert!(rule.validate(&config).is_ok());

        let mut config_without_system = SyneidesisConfig::default();
        config_without_system.system = None;
        assert!(rule.validate(&config_without_system).is_err());
    }

    #[test]
    fn test_custom_validator() {
        let mut validator = ConfigValidator::new();

        // Add custom validator
        validator.add_custom_validator("test_validator", |config| {
            if config.system.is_none() {
                Err(ValidationError::RequiredFieldMissing {
                    field: "system".to_string(),
                })
            } else {
                Ok(())
            }
        });

        let config = SyneidesisConfig::default();
        assert!(validator.validate(&config).is_ok());

        let mut config_without_system = SyneidesisConfig::default();
        config_without_system.system = None;
        assert!(validator.validate(&config_without_system).is_err());
    }
}
