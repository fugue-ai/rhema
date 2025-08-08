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

//! Error types for configuration management

use std::path::PathBuf;
use thiserror::Error;

/// Configuration error types
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Failed to read file {path}: {source}")]
    FileReadError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to parse {format} file {path}: {source}")]
    ParseError {
        format: String,
        path: PathBuf,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Invalid configuration format: {message}")]
    InvalidFormat { message: String },

    #[error("Missing required configuration: {field}")]
    MissingField { field: String },

    #[error("Invalid value for field {field}: {value}")]
    InvalidValue { field: String, value: String },

    #[error("Environment variable not found: {var}")]
    EnvVarNotFound { var: String },

    #[error("Failed to load environment variables: {source}")]
    EnvLoadError { source: std::env::VarError },

    #[error("Validation failed: {message}")]
    ValidationError { message: String },

    #[error("Configuration reload failed: {source}")]
    ReloadError { source: Box<ConfigError> },

    #[error("Configuration watcher error: {source}")]
    WatcherError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Configuration merge failed: {message}")]
    MergeError { message: String },

    #[error("Configuration serialization failed: {source}")]
    SerializationError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Configuration deserialization failed: {source}")]
    DeserializationError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Configuration type conversion failed: {message}")]
    TypeConversionError { message: String },

    #[error("Configuration path resolution failed: {path}")]
    PathResolutionError { path: String },

    #[error("Configuration template processing failed: {message}")]
    TemplateError { message: String },

    #[error("Configuration encryption/decryption failed: {message}")]
    CryptoError { message: String },

    #[error("Configuration backup/restore failed: {message}")]
    BackupError { message: String },

    #[error("Configuration migration failed: {message}")]
    MigrationError { message: String },

    #[error("Configuration schema error: {message}")]
    SchemaError { message: String },

    #[error("Configuration permission error: {path}")]
    PermissionError { path: PathBuf },

    #[error("Configuration timeout: {operation}")]
    TimeoutError { operation: String },

    #[error("Configuration resource exhausted: {resource}")]
    ResourceError { resource: String },

    #[error("Configuration network error: {source}")]
    NetworkError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Configuration database error: {source}")]
    DatabaseError {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Configuration cache error: {message}")]
    CacheError { message: String },

    #[error("Configuration lock error: {message}")]
    LockError { message: String },

    #[error("Configuration initialization failed: {message}")]
    InitError { message: String },

    #[error("Configuration shutdown failed: {message}")]
    ShutdownError { message: String },

    #[error("Unknown configuration error: {message}")]
    Unknown { message: String },
}

/// Validation error types
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Field validation failed: {field} - {message}")]
    FieldValidation { field: String, message: String },

    #[error("Schema validation failed: {message}")]
    SchemaValidation { message: String },

    #[error("Type validation failed: {field} - expected {expected}, got {actual}")]
    TypeValidation {
        field: String,
        expected: String,
        actual: String,
    },

    #[error("Range validation failed: {field} - value {value} not in range [{min}, {max}]")]
    RangeValidation {
        field: String,
        value: String,
        min: String,
        max: String,
    },

    #[error("Required field missing: {field}")]
    RequiredFieldMissing { field: String },

    #[error("Invalid enum value: {field} - value {value} not in allowed values {allowed:?}")]
    InvalidEnumValue {
        field: String,
        value: String,
        allowed: Vec<String>,
    },

    #[error("String validation failed: {field} - {message}")]
    StringValidation { field: String, message: String },

    #[error("Array validation failed: {field} - {message}")]
    ArrayValidation { field: String, message: String },

    #[error("Object validation failed: {field} - {message}")]
    ObjectValidation { field: String, message: String },

    #[error("Custom validation failed: {field} - {message}")]
    CustomValidation { field: String, message: String },

    #[error("Cross-field validation failed: {message}")]
    CrossFieldValidation { message: String },

    #[error("Dependency validation failed: {field} depends on {dependency}")]
    DependencyValidation { field: String, dependency: String },

    #[error("Conditional validation failed: {message}")]
    ConditionalValidation { message: String },

    #[error("Format validation failed: {field} - {message}")]
    FormatValidation { field: String, message: String },

    #[error("Pattern validation failed: {field} - value does not match pattern {pattern}")]
    PatternValidation { field: String, pattern: String },

    #[error("Length validation failed: {field} - length {length} not in range [{min}, {max}]")]
    LengthValidation {
        field: String,
        length: usize,
        min: usize,
        max: usize,
    },

    #[error("Size validation failed: {field} - size {size} not in range [{min}, {max}]")]
    SizeValidation {
        field: String,
        size: usize,
        min: usize,
        max: usize,
    },

    #[error("Date validation failed: {field} - {message}")]
    DateValidation { field: String, message: String },

    #[error("Email validation failed: {field} - {message}")]
    EmailValidation { field: String, message: String },

    #[error("URL validation failed: {field} - {message}")]
    UrlValidation { field: String, message: String },

    #[error("IP address validation failed: {field} - {message}")]
    IpValidation { field: String, message: String },

    #[error("MAC address validation failed: {field} - {message}")]
    MacValidation { field: String, message: String },

    #[error("UUID validation failed: {field} - {message}")]
    UuidValidation { field: String, message: String },

    #[error("Credit card validation failed: {field} - {message}")]
    CreditCardValidation { field: String, message: String },

    #[error("Phone number validation failed: {field} - {message}")]
    PhoneValidation { field: String, message: String },

    #[error("Postal code validation failed: {field} - {message}")]
    PostalCodeValidation { field: String, message: String },

    #[error("Country code validation failed: {field} - {message}")]
    CountryCodeValidation { field: String, message: String },

    #[error("Currency code validation failed: {field} - {message}")]
    CurrencyCodeValidation { field: String, message: String },

    #[error("Language code validation failed: {field} - {message}")]
    LanguageCodeValidation { field: String, message: String },

    #[error("Timezone validation failed: {field} - {message}")]
    TimezoneValidation { field: String, message: String },

    #[error("Color validation failed: {field} - {message}")]
    ColorValidation { field: String, message: String },

    #[error("File path validation failed: {field} - {message}")]
    FilePathValidation { field: String, message: String },

    #[error("Directory path validation failed: {field} - {message}")]
    DirectoryPathValidation { field: String, message: String },

    #[error("File extension validation failed: {field} - {message}")]
    FileExtensionValidation { field: String, message: String },

    #[error("MIME type validation failed: {field} - {message}")]
    MimeTypeValidation { field: String, message: String },

    #[error("Base64 validation failed: {field} - {message}")]
    Base64Validation { field: String, message: String },

    #[error("Hex validation failed: {field} - {message}")]
    HexValidation { field: String, message: String },

    #[error("JSON validation failed: {field} - {message}")]
    JsonValidation { field: String, message: String },

    #[error("XML validation failed: {field} - {message}")]
    XmlValidation { field: String, message: String },

    #[error("YAML validation failed: {field} - {message}")]
    YamlValidation { field: String, message: String },

    #[error("TOML validation failed: {field} - {message}")]
    TomlValidation { field: String, message: String },

    #[error("CSV validation failed: {field} - {message}")]
    CsvValidation { field: String, message: String },

    #[error("INI validation failed: {field} - {message}")]
    IniValidation { field: String, message: String },

    #[error("Multiple validation errors: {errors:?}")]
    MultipleErrors { errors: Vec<ValidationError> },
}

impl From<ValidationError> for ConfigError {
    fn from(error: ValidationError) -> Self {
        ConfigError::ValidationError {
            message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::FileReadError {
            path: PathBuf::from("unknown"),
            source: error,
        }
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(error: serde_json::Error) -> Self {
        ConfigError::ParseError {
            format: "JSON".to_string(),
            path: PathBuf::from("unknown"),
            source: Box::new(error),
        }
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        ConfigError::ParseError {
            format: "YAML".to_string(),
            path: PathBuf::from("unknown"),
            source: Box::new(error),
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        ConfigError::ParseError {
            format: "TOML".to_string(),
            path: PathBuf::from("unknown"),
            source: Box::new(error),
        }
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(error: toml::ser::Error) -> Self {
        ConfigError::SerializationError {
            source: Box::new(error),
        }
    }
}

impl From<std::env::VarError> for ConfigError {
    fn from(error: std::env::VarError) -> Self {
        ConfigError::EnvLoadError { source: error }
    }
}

impl From<validator::ValidationErrors> for ConfigError {
    fn from(errors: validator::ValidationErrors) -> Self {
        ConfigError::ValidationError {
            message: format!("Validation errors: {errors}"),
        }
    }
}
