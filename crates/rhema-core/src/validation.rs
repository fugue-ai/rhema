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

use crate::{RhemaError, RhemaResult};
use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Configuration for validation rules
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Allowed absolute paths that are considered safe
    pub allowed_absolute_paths: HashSet<PathBuf>,
    /// Whether to allow absolute paths in development mode
    pub allow_absolute_in_dev: bool,
    /// Whether to allow absolute paths in test mode
    pub allow_absolute_in_test: bool,
    /// Maximum path length
    pub max_path_length: usize,
    /// Whether to enable strict path validation
    pub strict_path_validation: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        let mut allowed_paths = HashSet::new();

        // Add common safe directories
        #[cfg(target_os = "linux")]
        {
            allowed_paths.insert(PathBuf::from("/tmp"));
            allowed_paths.insert(PathBuf::from("/var/tmp"));
            allowed_paths.insert(PathBuf::from("/usr/share"));
            allowed_paths.insert(PathBuf::from("/opt"));
        }

        #[cfg(target_os = "macos")]
        {
            allowed_paths.insert(PathBuf::from("/tmp"));
            allowed_paths.insert(PathBuf::from("/var/tmp"));
            allowed_paths.insert(PathBuf::from("/Applications"));
            allowed_paths.insert(PathBuf::from("/usr/local"));
        }

        #[cfg(target_os = "windows")]
        {
            allowed_paths.insert(PathBuf::from("C:\\Windows\\Temp"));
            allowed_paths.insert(PathBuf::from("C:\\Program Files"));
            allowed_paths.insert(PathBuf::from("C:\\Program Files (x86)"));
        }

        Self {
            allowed_absolute_paths: allowed_paths,
            allow_absolute_in_dev: cfg!(debug_assertions),
            allow_absolute_in_test: true,
            max_path_length: 4096,
            strict_path_validation: true,
        }
    }
}

/// Validation rules for Rhema data structures
pub struct ValidationRules {
    config: ValidationConfig,
}

impl ValidationRules {
    /// Create a new ValidationRules instance with default configuration
    pub fn new() -> Self {
        Self {
            config: ValidationConfig::default(),
        }
    }

    /// Create a default ValidationRules instance (for backward compatibility)
    pub fn default() -> Self {
        Self::new()
    }

    /// Create a new ValidationRules instance with custom configuration
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }

    /// Get the current configuration
    pub fn config(&self) -> &ValidationConfig {
        &self.config
    }

    /// Update the configuration
    pub fn update_config(&mut self, config: ValidationConfig) {
        self.config = config;
    }

    /// Add an allowed absolute path
    pub fn add_allowed_absolute_path(&mut self, path: PathBuf) {
        self.config.allowed_absolute_paths.insert(path);
    }

    /// Remove an allowed absolute path
    pub fn remove_allowed_absolute_path(&mut self, path: &Path) {
        self.config.allowed_absolute_paths.remove(path);
    }

    /// Validate a file path for security (static method for backward compatibility)
    pub fn validate_file_path_static(path: &Path) -> RhemaResult<()> {
        let rules = Self::new();
        rules.validate_file_path(path)
    }

    /// Validate a title string (static method for backward compatibility)
    pub fn validate_title_static(title: &str) -> RhemaResult<()> {
        let rules = Self::new();
        rules.validate_title(title)
    }

    /// Validate a scope path (static method for backward compatibility)
    pub fn validate_scope_path_static(scope_path: &Path) -> RhemaResult<()> {
        let rules = Self::new();
        rules.validate_scope_path(scope_path)
    }

    /// Validate a file path for security
    pub fn validate_file_path(&self, path: &Path) -> RhemaResult<()> {
        let path_str = path.to_string_lossy();

        // Check path length
        if path_str.len() > self.config.max_path_length {
            return Err(RhemaError::ValidationError(format!(
                "Path too long (max {} characters)",
                self.config.max_path_length
            )));
        }

        // Check for path traversal attempts
        if self.config.strict_path_validation
            && (path_str.contains("..") || path_str.contains("//"))
        {
            return Err(RhemaError::SecurityError(
                "Path traversal attempt detected".to_string(),
            ));
        }

        // Check for absolute paths outside of allowed directories
        if path.is_absolute() {
            // Allow absolute paths in test environments
            #[cfg(test)]
            if self.config.allow_absolute_in_test {
                // In tests, allow absolute paths for temp directories
                if path_str.contains("target") || path_str.contains("tmp") {
                    return Ok(());
                }
            }

            // Allow absolute paths in development mode
            #[cfg(debug_assertions)]
            if self.config.allow_absolute_in_dev {
                return Ok(());
            }

            // Check if the absolute path is in the allowed list
            if self
                .config
                .allowed_absolute_paths
                .iter()
                .any(|allowed_path| path.starts_with(allowed_path))
            {
                return Ok(());
            }

            return Err(RhemaError::SecurityError(format!(
                "Absolute path not allowed: {}",
                path.display()
            )));
        }

        Ok(())
    }

    /// Validate a title string
    pub fn validate_title(&self, title: &str) -> RhemaResult<()> {
        if title.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "Title cannot be empty".to_string(),
            ));
        }

        if title.len() > 200 {
            return Err(RhemaError::ValidationError(
                "Title too long (max 200 characters)".to_string(),
            ));
        }

        // Check for potentially dangerous characters
        let dangerous_chars = ['<', '>', '&', '"', '\'', '\\', '/'];
        if dangerous_chars.iter().any(|&c| title.contains(c)) {
            return Err(RhemaError::ValidationError(
                "Title contains invalid characters".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate a description string
    pub fn validate_description(&self, description: &str) -> RhemaResult<()> {
        if description.len() > 10000 {
            return Err(RhemaError::ValidationError(
                "Description too long (max 10000 characters)".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate an ID string
    pub fn validate_id(&self, id: &str) -> RhemaResult<()> {
        if id.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "ID cannot be empty".to_string(),
            ));
        }

        if id.len() > 100 {
            return Err(RhemaError::ValidationError(
                "ID too long (max 100 characters)".to_string(),
            ));
        }

        // Check for valid ID format (alphanumeric, hyphens, underscores)
        let id_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        if !id_regex.is_match(id) {
            return Err(RhemaError::ValidationError(
                "ID contains invalid characters (only alphanumeric, hyphens, and underscores allowed)".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate a date string
    pub fn validate_date_string(&self, date_str: &str) -> RhemaResult<()> {
        if chrono::DateTime::parse_from_rfc3339(date_str).is_err() {
            return Err(RhemaError::ValidationError(
                "Invalid date format. Use ISO 8601 format (e.g., 2023-12-01T10:00:00Z)".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate a priority value
    pub fn validate_priority(&self, priority: i32) -> RhemaResult<()> {
        if priority < 1 || priority > 5 {
            return Err(RhemaError::ValidationError(
                "Priority must be between 1 and 5".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate a URL string
    pub fn validate_url(&self, url: &str) -> RhemaResult<()> {
        if url.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "URL cannot be empty".to_string(),
            ));
        }

        // Basic URL validation
        let url_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
        if !url_regex.is_match(url) {
            return Err(RhemaError::ValidationError(
                "Invalid URL format".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate an email address
    pub fn validate_email(&self, email: &str) -> RhemaResult<()> {
        if email.trim().is_empty() {
            return Err(RhemaError::ValidationError(
                "Email cannot be empty".to_string(),
            ));
        }

        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(email) {
            return Err(RhemaError::ValidationError(
                "Invalid email format".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate a scope path
    pub fn validate_scope_path(&self, scope_path: &Path) -> RhemaResult<()> {
        self.validate_file_path(scope_path)?;

        // Check if scope path exists and is a directory
        if !scope_path.exists() {
            return Err(RhemaError::NotFound(format!(
                "Scope path does not exist: {}",
                scope_path.display()
            )));
        }

        if !scope_path.is_dir() {
            return Err(RhemaError::ValidationError(format!(
                "Scope path is not a directory: {}",
                scope_path.display()
            )));
        }

        Ok(())
    }
}

/// Trait for validating Rhema data structures
pub trait RhemaValidate {
    fn validate(&self) -> RhemaResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_validate_title() {
        let rules = ValidationRules::new();
        assert!(rules.validate_title("Valid Title").is_ok());
        assert!(rules.validate_title("").is_err());
        assert!(rules.validate_title(&"a".repeat(201)).is_err());
        assert!(rules.validate_title("Title with <script>").is_err());
    }

    #[test]
    fn test_validate_id() {
        let rules = ValidationRules::new();
        assert!(rules.validate_id("valid-id_123").is_ok());
        assert!(rules.validate_id("").is_err());
        assert!(rules.validate_id("invalid id").is_err());
        assert!(rules.validate_id("invalid@id").is_err());
    }

    #[test]
    fn test_validate_priority() {
        let rules = ValidationRules::new();
        assert!(rules.validate_priority(1).is_ok());
        assert!(rules.validate_priority(5).is_ok());
        assert!(rules.validate_priority(0).is_err());
        assert!(rules.validate_priority(6).is_err());
    }

    #[test]
    fn test_validate_date_string() {
        let rules = ValidationRules::new();
        assert!(rules.validate_date_string("2023-12-01T10:00:00Z").is_ok());
        assert!(rules.validate_date_string("invalid-date").is_err());
    }

    #[test]
    fn test_validate_url() {
        let rules = ValidationRules::new();
        assert!(rules.validate_url("https://example.com").is_ok());
        assert!(rules.validate_url("http://example.com/path").is_ok());
        assert!(rules.validate_url("").is_err());
        assert!(rules.validate_url("not-a-url").is_err());
    }

    #[test]
    fn test_validate_email() {
        let rules = ValidationRules::new();
        assert!(rules.validate_email("test@example.com").is_ok());
        assert!(rules.validate_email("").is_err());
        assert!(rules.validate_email("invalid-email").is_err());
    }

    #[test]
    fn test_validate_file_path() {
        let rules = ValidationRules::new();
        assert!(rules.validate_file_path(Path::new("valid/path")).is_ok());
        assert!(rules
            .validate_file_path(Path::new("path/../traversal"))
            .is_err());
        assert!(rules
            .validate_file_path(Path::new("path//double-slash"))
            .is_err());
    }

    #[test]
    fn test_validation_config() {
        let mut config = ValidationConfig::default();

        // Test adding allowed paths
        config
            .allowed_absolute_paths
            .insert(PathBuf::from("/custom/path"));
        assert!(config
            .allowed_absolute_paths
            .contains(&PathBuf::from("/custom/path")));

        // Test configuration options
        assert_eq!(config.max_path_length, 4096);
        assert!(config.strict_path_validation);
    }

    #[test]
    fn test_validation_rules_with_config() {
        let mut config = ValidationConfig::default();
        config
            .allowed_absolute_paths
            .insert(PathBuf::from("/allowed/path"));
        config.strict_path_validation = false;

        let rules = ValidationRules::with_config(config);

        // Test that allowed absolute paths are accepted
        assert!(rules
            .validate_file_path(Path::new("/allowed/path/file.txt"))
            .is_ok());

        // Test that non-allowed absolute paths are still rejected
        assert!(rules
            .validate_file_path(Path::new("/forbidden/path"))
            .is_err());
    }

    #[test]
    fn test_validation_rules_configuration() {
        let mut rules = ValidationRules::new();

        // Test adding allowed paths
        rules.add_allowed_absolute_path(PathBuf::from("/test/path"));
        assert!(rules
            .config()
            .allowed_absolute_paths
            .contains(&PathBuf::from("/test/path")));

        // Test removing allowed paths
        rules.remove_allowed_absolute_path(Path::new("/test/path"));
        assert!(!rules
            .config()
            .allowed_absolute_paths
            .contains(&PathBuf::from("/test/path")));
    }

    #[test]
    fn test_static_validation_methods() {
        // Test static methods for backward compatibility
        assert!(ValidationRules::validate_file_path_static(Path::new("valid/path")).is_ok());
        assert!(ValidationRules::validate_title_static("Valid Title").is_ok());
        assert!(ValidationRules::validate_scope_path_static(Path::new("valid/scope")).is_ok());
    }

    #[test]
    fn test_absolute_path_validation() {
        let mut rules = ValidationRules::new();

        // Test that absolute paths are rejected by default
        assert!(rules.validate_file_path(Path::new("/etc/passwd")).is_err());
        assert!(rules
            .validate_file_path(Path::new("/home/user/file.txt"))
            .is_err());

        // Test that allowed absolute paths are accepted
        rules.add_allowed_absolute_path(PathBuf::from("/tmp"));
        assert!(rules.validate_file_path(Path::new("/tmp/file.txt")).is_ok());
        assert!(rules
            .validate_file_path(Path::new("/tmp/subdir/file.txt"))
            .is_ok());

        // Test that other absolute paths are still rejected
        assert!(rules.validate_file_path(Path::new("/etc/passwd")).is_err());
    }
}
