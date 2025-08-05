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

use chrono::{DateTime, Utc};
use git2::{Repository, Commit};
use regex::Regex;
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use semver::Version;

/// Version management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManagementConfig {
    /// Versioning strategy
    pub strategy: VersioningStrategy,
    
    /// Version file paths
    pub version_files: Vec<VersionFile>,
    
    /// Changelog configuration
    pub changelog: ChangelogConfig,
    
    /// Release notes configuration
    pub release_notes: ReleaseNotesConfig,
    
    /// Commit message patterns for version bumping
    pub commit_patterns: CommitPatterns,
    
    /// Automated version bumping settings
    pub auto_bump: AutoBumpConfig,
    
    /// Version validation rules
    pub validation: VersionValidationConfig,
}

/// Versioning strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersioningStrategy {
    /// Semantic versioning (MAJOR.MINOR.PATCH)
    Semantic,
    /// Calendar versioning (YYYY.MM.DD)
    Calendar,
    /// Custom versioning pattern
    Custom(String),
}

/// Version file configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionFile {
    /// File path
    pub path: PathBuf,
    
    /// File type
    pub file_type: VersionFileType,
    
    /// Version pattern/regex
    pub version_pattern: String,
    
    /// Replacement pattern
    pub replacement_pattern: String,
    
    /// Whether this file is required
    pub required: bool,
}

/// Version file types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionFileType {
    /// Cargo.toml file
    CargoToml,
    /// package.json file
    PackageJson,
    /// Version file (version.txt, VERSION, etc.)
    VersionFile,
    /// Custom file type
    Custom(String),
}

/// Changelog configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogConfig {
    /// Changelog file path
    pub file_path: PathBuf,
    
    /// Changelog format
    pub format: ChangelogFormat,
    
    /// Include commit hashes
    pub include_commit_hashes: bool,
    
    /// Include author information
    pub include_author: bool,
    
    /// Include date information
    pub include_date: bool,
    
    /// Group commits by type
    pub group_by_type: bool,
    
    /// Commit type mapping
    pub commit_types: HashMap<String, String>,
    
    /// Template for changelog entries
    pub template: Option<String>,
}

/// Changelog format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangelogFormat {
    /// Markdown format
    Markdown,
    /// JSON format
    Json,
    /// Custom format
    Custom(String),
}

/// Release notes configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNotesConfig {
    /// Release notes directory
    pub directory: PathBuf,
    
    /// Release notes format
    pub format: ReleaseNotesFormat,
    
    /// Include breaking changes section
    pub include_breaking_changes: bool,
    
    /// Include migration guide
    pub include_migration_guide: bool,
    
    /// Include security notes
    pub include_security_notes: bool,
    
    /// Template for release notes
    pub template: Option<String>,
}

/// Release notes format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReleaseNotesFormat {
    /// Markdown format
    Markdown,
    /// HTML format
    Html,
    /// Custom format
    Custom(String),
}

/// Commit message patterns for version bumping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitPatterns {
    /// Major version bump patterns
    pub major_bump: Vec<String>,
    
    /// Minor version bump patterns
    pub minor_bump: Vec<String>,
    
    /// Patch version bump patterns
    pub patch_bump: Vec<String>,
    
    /// Breaking change patterns
    pub breaking_change: Vec<String>,
    
    /// Ignore patterns (commits that don't affect version)
    pub ignore: Vec<String>,
}

/// Automated version bumping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoBumpConfig {
    /// Enable automatic version bumping
    pub enabled: bool,
    
    /// Bump strategy
    pub strategy: BumpStrategy,
    
    /// Analyze commit messages for version bumping
    pub analyze_commits: bool,
    
    /// Analyze file changes for version bumping
    pub analyze_changes: bool,
    
    /// Minimum confidence for auto-bump
    pub min_confidence: f64,
    
    /// Require confirmation for major version bumps
    pub confirm_major_bumps: bool,
}

/// Bump strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BumpStrategy {
    /// Conservative - only bump when explicitly indicated
    Conservative,
    /// Aggressive - bump based on commit analysis
    Aggressive,
    /// Custom strategy
    Custom(String),
}

/// Version validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionValidationConfig {
    /// Validate semantic versioning
    pub validate_semver: bool,
    
    /// Require version consistency across files
    pub require_consistency: bool,
    
    /// Validate version format
    pub validate_format: bool,
    
    /// Custom validation rules
    pub custom_rules: Vec<ValidationRule>,
}

/// Validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    
    /// Rule description
    pub description: String,
    
    /// Validation pattern
    pub pattern: String,
    
    /// Error message
    pub error_message: String,
    
    /// Whether this rule is required
    pub required: bool,
}

/// Version management result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManagementResult {
    /// Success status
    pub success: bool,
    
    /// Old version
    pub old_version: Option<String>,
    
    /// New version
    pub new_version: Option<String>,
    
    /// Bump type
    pub bump_type: Option<BumpType>,
    
    /// Changelog generated
    pub changelog_generated: bool,
    
    /// Release notes generated
    pub release_notes_generated: bool,
    
    /// Messages
    pub messages: Vec<String>,
    
    /// Errors
    pub errors: Vec<String>,
}

/// Bump type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BumpType {
    /// Major version bump
    Major,
    /// Minor version bump
    Minor,
    /// Patch version bump
    Patch,
    /// No bump needed
    None,
}

/// Commit information for changelog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    /// Commit hash
    pub hash: String,
    
    /// Commit message
    pub message: String,
    
    /// Author name
    pub author: String,
    
    /// Author email
    pub email: String,
    
    /// Commit date
    pub date: DateTime<Utc>,
    
    /// Commit type
    pub commit_type: CommitType,
    
    /// Breaking change
    pub breaking_change: bool,
    
    /// Scope
    pub scope: Option<String>,
    
    /// Description
    pub description: String,
}

/// Commit type
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum CommitType {
    /// Feature commit
    Feature,
    /// Bug fix commit
    Fix,
    /// Documentation commit
    Docs,
    /// Style commit
    Style,
    /// Refactor commit
    Refactor,
    /// Test commit
    Test,
    /// Chore commit
    Chore,
    /// Breaking change commit
    Breaking,
    /// Custom commit type
    Custom(String),
}

/// Version manager
pub struct VersionManager {
    pub(crate) repo: Repository,
    config: VersionManagementConfig,
}

impl VersionManager {
    /// Create a new version manager
    pub fn new(repo: Repository, config: VersionManagementConfig) -> Self {
        Self { repo, config }
    }

    /// Get current version from version files
    pub fn get_current_version(&self) -> RhemaResult<String> {
        for version_file in &self.config.version_files {
            if let Ok(version) = self.read_version_from_file(&version_file.path, &version_file.file_type) {
                return Ok(version);
            }
        }
        
        Err(RhemaError::ConfigError("No version found in any version file".to_string()))
    }

    /// Bump version based on strategy and commit analysis
    pub async fn bump_version(&self, bump_type: Option<BumpType>) -> RhemaResult<VersionManagementResult> {
        let current_version = self.get_current_version()?;
        let bump_type = match bump_type {
            Some(bump) => bump,
            None => self.determine_bump_type(&current_version).await?,
        };

        let new_version = match bump_type {
            BumpType::Major => self.bump_major_version(&current_version)?,
            BumpType::Minor => self.bump_minor_version(&current_version)?,
            BumpType::Patch => self.bump_patch_version(&current_version)?,
            BumpType::None => current_version.clone(),
        };

        let mut result = VersionManagementResult {
            success: true,
            old_version: Some(current_version.clone()),
            new_version: Some(new_version.clone()),
            bump_type: Some(bump_type.clone()),
            changelog_generated: false,
            release_notes_generated: false,
            messages: vec![],
            errors: vec![],
        };

        // Update version files
        if bump_type != BumpType::None {
            self.update_version_files(&current_version, &new_version)?;
            result.messages.push(format!("Updated version from {} to {}", current_version, new_version));
        }

        // Generate changelog
        if self.config.changelog.file_path.exists() || self.config.changelog.file_path.parent().map(|p| p.exists()).unwrap_or(false) {
            self.generate_changelog(&new_version).await?;
            result.changelog_generated = true;
            result.messages.push("Generated changelog".to_string());
        }

        // Generate release notes
        if self.config.release_notes.directory.exists() {
            self.generate_release_notes(&new_version).await?;
            result.release_notes_generated = true;
            result.messages.push("Generated release notes".to_string());
        }

        Ok(result)
    }

    /// Generate changelog for a version
    pub async fn generate_changelog(&self, version: &str) -> RhemaResult<()> {
        let commits = self.get_commits_since_last_tag().await?;
        let changelog_content = self.format_changelog(version, &commits).await?;
        
        // Ensure directory exists
        if let Some(parent) = self.config.changelog.file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Write changelog
        let mut file = fs::File::create(&self.config.changelog.file_path)?;
        file.write_all(changelog_content.as_bytes())?;
        
        Ok(())
    }

    /// Generate release notes for a version
    pub async fn generate_release_notes(&self, version: &str) -> RhemaResult<()> {
        let commits = self.get_commits_since_last_tag().await?;
        let release_notes_content = self.format_release_notes(version, &commits).await?;
        
        // Ensure directory exists
        fs::create_dir_all(&self.config.release_notes.directory)?;
        
        // Write release notes
        let release_notes_path = self.config.release_notes.directory.join(format!("release-{}.md", version));
        let mut file = fs::File::create(release_notes_path)?;
        file.write_all(release_notes_content.as_bytes())?;
        
        Ok(())
    }

    /// Validate version format and consistency
    pub fn validate_version(&self, version: &str) -> RhemaResult<Vec<String>> {
        let mut errors = Vec::new();

        // Validate semantic versioning if enabled
        if self.config.validation.validate_semver {
            if let Err(_) = Version::parse(version) {
                errors.push(format!("Invalid semantic version format: {}", version));
            }
        }

        // Validate version consistency across files
        if self.config.validation.require_consistency {
            let mut versions = Vec::new();
            for version_file in &self.config.version_files {
                if let Ok(ver) = self.read_version_from_file(&version_file.path, &version_file.file_type) {
                    versions.push(ver);
                }
            }
            
            if versions.len() > 1 {
                let first_version = &versions[0];
                for (i, ver) in versions.iter().enumerate().skip(1) {
                    if ver != first_version {
                        errors.push(format!("Version inconsistency detected: {} vs {}", first_version, ver));
                    }
                }
            }
        }

        // Run custom validation rules
        for rule in &self.config.validation.custom_rules {
            let regex = Regex::new(&rule.pattern).map_err(|e| RhemaError::ConfigError(format!("Invalid regex pattern: {}", e)))?;
            if !regex.is_match(version) {
                errors.push(rule.error_message.clone());
            }
        }

        Ok(errors)
    }

    /// Read version from a file
    fn read_version_from_file(&self, path: &Path, file_type: &VersionFileType) -> RhemaResult<String> {
        if !path.exists() {
            return Err(RhemaError::ConfigError(format!("Version file not found: {:?}", path)));
        }

        let content = fs::read_to_string(path)?;
        
        match file_type {
            VersionFileType::CargoToml => self.parse_cargo_toml_version(&content),
            VersionFileType::PackageJson => self.parse_package_json_version(&content),
            VersionFileType::VersionFile => self.parse_version_file(&content),
            VersionFileType::Custom(_) => self.parse_custom_version_file(&content, file_type),
        }
    }

    /// Parse version from Cargo.toml
    fn parse_cargo_toml_version(&self, content: &str) -> RhemaResult<String> {
        let version_regex = Regex::new(r#"version\s*=\s*"([^"]+)""#).map_err(|e| RhemaError::ConfigError(format!("Invalid regex: {}", e)))?;
        if let Some(captures) = version_regex.captures(content) {
            Ok(captures[1].to_string())
        } else {
            Err(RhemaError::ConfigError("Version not found in Cargo.toml".to_string()))
        }
    }

    /// Parse version from package.json
    fn parse_package_json_version(&self, content: &str) -> RhemaResult<String> {
        let version_regex = Regex::new(r#""version"\s*:\s*"([^"]+)""#).map_err(|e| RhemaError::ConfigError(format!("Invalid regex: {}", e)))?;
        if let Some(captures) = version_regex.captures(content) {
            Ok(captures[1].to_string())
        } else {
            Err(RhemaError::ConfigError("Version not found in package.json".to_string()))
        }
    }

    /// Parse version from version file
    fn parse_version_file(&self, content: &str) -> RhemaResult<String> {
        let version = content.trim();
        if version.is_empty() {
            Err(RhemaError::ConfigError("Version file is empty".to_string()))
        } else {
            Ok(version.to_string())
        }
    }

    /// Parse version from custom file type
    fn parse_custom_version_file(&self, content: &str, file_type: &VersionFileType) -> RhemaResult<String> {
        // Default implementation - can be overridden for specific file types
        self.parse_version_file(content)
    }

    /// Update version in all version files
    fn update_version_files(&self, old_version: &str, new_version: &str) -> RhemaResult<()> {
        for version_file in &self.config.version_files {
            self.update_version_in_file(&version_file.path, &version_file.file_type, old_version, new_version)?;
        }
        Ok(())
    }

    /// Update version in a specific file
    fn update_version_in_file(&self, path: &Path, file_type: &VersionFileType, old_version: &str, new_version: &str) -> RhemaResult<()> {
        if !path.exists() {
            return Err(RhemaError::ConfigError(format!("Version file not found: {:?}", path)));
        }

        let content = fs::read_to_string(path)?;
        let updated_content = match file_type {
            VersionFileType::CargoToml => self.update_cargo_toml_version(&content, old_version, new_version)?,
            VersionFileType::PackageJson => self.update_package_json_version(&content, old_version, new_version)?,
            VersionFileType::VersionFile => self.update_version_file_content(&content, old_version, new_version)?,
            VersionFileType::Custom(_) => self.update_custom_version_file(&content, file_type, old_version, new_version)?,
        };

        fs::write(path, updated_content)?;
        Ok(())
    }

    /// Update version in Cargo.toml
    fn update_cargo_toml_version(&self, content: &str, old_version: &str, new_version: &str) -> RhemaResult<String> {
        let pattern = format!(r#"version\s*=\s*"{}""#, regex::escape(old_version));
        let replacement = format!(r#"version = "{}""#, new_version);
        let regex = Regex::new(&pattern).map_err(|e| RhemaError::ConfigError(format!("Invalid regex: {}", e)))?;
        
        if regex.is_match(content) {
            Ok(regex.replace_all(content, replacement).to_string())
        } else {
            Err(RhemaError::ConfigError("Version not found in Cargo.toml".to_string()))
        }
    }

    /// Update version in package.json
    fn update_package_json_version(&self, content: &str, old_version: &str, new_version: &str) -> RhemaResult<String> {
        let pattern = format!(r#""version"\s*:\s*"{}""#, regex::escape(old_version));
        let replacement = format!(r#""version": "{}""#, new_version);
        let regex = Regex::new(&pattern).map_err(|e| RhemaError::ConfigError(format!("Invalid regex: {}", e)))?;
        
        if regex.is_match(content) {
            Ok(regex.replace_all(content, replacement).to_string())
        } else {
            Err(RhemaError::ConfigError("Version not found in package.json".to_string()))
        }
    }

    /// Update version in version file
    fn update_version_file_content(&self, content: &str, old_version: &str, new_version: &str) -> RhemaResult<String> {
        let pattern = regex::escape(old_version);
        let regex = Regex::new(&pattern).map_err(|e| RhemaError::ConfigError(format!("Invalid regex: {}", e)))?;
        
        if regex.is_match(content) {
            Ok(regex.replace_all(content, new_version).to_string())
        } else {
            Err(RhemaError::ConfigError("Version not found in version file".to_string()))
        }
    }

    /// Update version in custom file type
    fn update_custom_version_file(&self, content: &str, file_type: &VersionFileType, old_version: &str, new_version: &str) -> RhemaResult<String> {
        // Default implementation - can be overridden for specific file types
        self.update_version_file_content(content, old_version, new_version)
    }

    /// Determine bump type based on commit analysis
    async fn determine_bump_type(&self, _current_version: &str) -> RhemaResult<BumpType> {
        if !self.config.auto_bump.enabled {
            return Ok(BumpType::None);
        }

        let commits = self.get_commits_since_last_tag().await?;
        let mut major_bumps = 0;
        let mut minor_bumps = 0;
        let mut patch_bumps = 0;

        for commit in &commits {
            if commit.breaking_change {
                major_bumps += 1;
            } else {
                match commit.commit_type {
                    CommitType::Feature => minor_bumps += 1,
                    CommitType::Fix => patch_bumps += 1,
                    CommitType::Breaking => major_bumps += 1,
                    _ => patch_bumps += 1,
                }
            }
        }

        if major_bumps > 0 {
            Ok(BumpType::Major)
        } else if minor_bumps > 0 {
            Ok(BumpType::Minor)
        } else if patch_bumps > 0 {
            Ok(BumpType::Patch)
        } else {
            Ok(BumpType::None)
        }
    }

    /// Bump major version
    fn bump_major_version(&self, version: &str) -> RhemaResult<String> {
        let mut ver = Version::parse(version).map_err(|e| RhemaError::ConfigError(format!("Invalid version format: {}", e)))?;
        ver.major += 1;
        ver.minor = 0;
        ver.patch = 0;
        ver.pre = semver::Prerelease::EMPTY;
        ver.build = semver::BuildMetadata::EMPTY;
        Ok(ver.to_string())
    }

    /// Bump minor version
    fn bump_minor_version(&self, version: &str) -> RhemaResult<String> {
        let mut ver = Version::parse(version).map_err(|e| RhemaError::ConfigError(format!("Invalid version format: {}", e)))?;
        ver.minor += 1;
        ver.patch = 0;
        ver.pre = semver::Prerelease::EMPTY;
        ver.build = semver::BuildMetadata::EMPTY;
        Ok(ver.to_string())
    }

    /// Bump patch version
    fn bump_patch_version(&self, version: &str) -> RhemaResult<String> {
        let mut ver = Version::parse(version).map_err(|e| RhemaError::ConfigError(format!("Invalid version format: {}", e)))?;
        ver.patch += 1;
        ver.pre = semver::Prerelease::EMPTY;
        ver.build = semver::BuildMetadata::EMPTY;
        Ok(ver.to_string())
    }

    /// Get commits since last tag
    async fn get_commits_since_last_tag(&self) -> RhemaResult<Vec<CommitInfo>> {
        let mut commits = Vec::new();
        
        // Get the last tag
        let tags = self.repo.tag_names(None)?;
        let last_tag = tags.iter().filter_map(|t| t).last();
        
        let mut revwalk = self.repo.revwalk()?;
        if let Some(last_tag) = last_tag {
            revwalk.push_range(&format!("{}..HEAD", last_tag))?;
        } else {
            revwalk.push_head()?;
        }

        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            
            let commit_info = self.parse_commit_info(&commit)?;
            commits.push(commit_info);
        }

        Ok(commits)
    }

    /// Parse commit information
    fn parse_commit_info(&self, commit: &Commit) -> RhemaResult<CommitInfo> {
        let message = commit.message().unwrap_or("").to_string();
        let author = commit.author();
        
        let (commit_type, scope, description, breaking_change) = self.parse_commit_message(&message);
        
        Ok(CommitInfo {
            hash: commit.id().to_string(),
            message: message.clone(),
            author: author.name().unwrap_or("Unknown").to_string(),
            email: author.email().unwrap_or("").to_string(),
            date: DateTime::from_timestamp(commit.time().seconds(), 0).unwrap_or_else(|| Utc::now()),
            commit_type,
            breaking_change,
            scope,
            description,
        })
    }

    /// Parse commit message to extract type, scope, and description
    fn parse_commit_message(&self, message: &str) -> (CommitType, Option<String>, String, bool) {
        // Conventional commit format: type(scope): description
        let conventional_regex = Regex::new(r"^(\w+)(?:\(([^)]+)\))?(!)?:\s*(.+)$").unwrap();
        
        if let Some(captures) = conventional_regex.captures(message) {
            let commit_type = match captures.get(1).unwrap().as_str().to_lowercase().as_str() {
                "feat" | "feature" => CommitType::Feature,
                "fix" | "bugfix" => CommitType::Fix,
                "docs" | "documentation" => CommitType::Docs,
                "style" => CommitType::Style,
                "refactor" => CommitType::Refactor,
                "test" | "tests" => CommitType::Test,
                "chore" => CommitType::Chore,
                "breaking" => CommitType::Breaking,
                _ => CommitType::Custom(captures.get(1).unwrap().as_str().to_string()),
            };
            
            let scope = captures.get(2).map(|s| s.as_str().to_string());
            let breaking_change = captures.get(3).is_some();
            let description = captures.get(4).unwrap().as_str().to_string();
            
            (commit_type, scope, description, breaking_change)
        } else {
            // Fallback: try to determine type from message content
            let message_lower = message.to_lowercase();
            let commit_type = if message_lower.contains("feat") || message_lower.contains("feature") {
                CommitType::Feature
            } else if message_lower.contains("fix") || message_lower.contains("bug") {
                CommitType::Fix
            } else if message_lower.contains("docs") || message_lower.contains("documentation") {
                CommitType::Docs
            } else if message_lower.contains("refactor") {
                CommitType::Refactor
            } else if message_lower.contains("test") {
                CommitType::Test
            } else {
                CommitType::Chore
            };
            
            let breaking_change = message_lower.contains("breaking") || message_lower.contains("!breaking");
            
            (commit_type, None, message.to_string(), breaking_change)
        }
    }

    /// Format changelog content
    async fn format_changelog(&self, version: &str, commits: &[CommitInfo]) -> RhemaResult<String> {
        match self.config.changelog.format {
            ChangelogFormat::Markdown => self.format_markdown_changelog(version, commits).await,
            ChangelogFormat::Json => self.format_json_changelog(version, commits).await,
            ChangelogFormat::Custom(_) => self.format_custom_changelog(version, commits).await,
        }
    }

    /// Format markdown changelog
    async fn format_markdown_changelog(&self, version: &str, commits: &[CommitInfo]) -> RhemaResult<String> {
        let mut changelog = String::new();
        
        // Header
        changelog.push_str(&format!("# Changelog\n\n"));
        changelog.push_str(&format!("## [{}] - {}\n\n", version, Utc::now().format("%Y-%m-%d")));
        
        if self.config.changelog.group_by_type {
            // Group commits by type
            let mut grouped_commits: HashMap<CommitType, Vec<&CommitInfo>> = HashMap::new();
            
            for commit in commits {
                grouped_commits.entry(commit.commit_type.clone()).or_insert_with(Vec::new).push(commit);
            }
            
            // Add sections for each type
            let type_order = vec![
                CommitType::Breaking,
                CommitType::Feature,
                CommitType::Fix,
                CommitType::Refactor,
                CommitType::Docs,
                CommitType::Style,
                CommitType::Test,
                CommitType::Chore,
            ];
            
            for commit_type in type_order {
                if let Some(type_commits) = grouped_commits.get(&commit_type) {
                    let type_name = match commit_type {
                        CommitType::Breaking => "### Breaking Changes",
                        CommitType::Feature => "### Added",
                        CommitType::Fix => "### Fixed",
                        CommitType::Refactor => "### Changed",
                        CommitType::Docs => "### Documentation",
                        CommitType::Style => "### Style",
                        CommitType::Test => "### Tests",
                        CommitType::Chore => "### Chores",
                        CommitType::Custom(ref name) => &format!("### {}", name),
                    };
                    
                    changelog.push_str(&format!("{}\n\n", type_name));
                    
                    for commit in type_commits {
                        let mut entry = format!("- {}", commit.description);
                        
                        if self.config.changelog.include_date {
                            if let Some(ref scope) = commit.scope {
                                entry = format!("- **{}**: {}", scope, commit.description);
                            }
                        }
                        
                        if self.config.changelog.include_commit_hashes {
                            entry.push_str(&format!(" ({})", &commit.hash[..8]));
                        }
                        
                        if self.config.changelog.include_author {
                            entry.push_str(&format!(" - {}", commit.author));
                        }
                        
                        changelog.push_str(&format!("{}\n", entry));
                    }
                    
                    changelog.push('\n');
                }
            }
        } else {
            // List all commits chronologically
            for commit in commits {
                let mut entry = format!("- {}", commit.description);
                
                if self.config.changelog.include_commit_hashes {
                    entry.push_str(&format!(" ({})", &commit.hash[..8]));
                }
                
                if self.config.changelog.include_author {
                    entry.push_str(&format!(" - {}", commit.author));
                }
                
                if self.config.changelog.include_date {
                    entry.push_str(&format!(" - {}", commit.date.format("%Y-%m-%d")));
                }
                
                changelog.push_str(&format!("{}\n", entry));
            }
        }
        
        Ok(changelog)
    }

    /// Format JSON changelog
    async fn format_json_changelog(&self, version: &str, commits: &[CommitInfo]) -> RhemaResult<String> {
        let changelog_data = serde_json::json!({
            "version": version,
            "date": Utc::now().to_rfc3339(),
            "commits": commits
        });
        
        Ok(serde_json::to_string_pretty(&changelog_data)?)
    }

    /// Format custom changelog
    async fn format_custom_changelog(&self, version: &str, commits: &[CommitInfo]) -> RhemaResult<String> {
        // Default to markdown format for custom
        self.format_markdown_changelog(version, commits).await
    }

    /// Format release notes
    async fn format_release_notes(&self, version: &str, commits: &[CommitInfo]) -> RhemaResult<String> {
        match self.config.release_notes.format {
            ReleaseNotesFormat::Markdown => self.format_markdown_release_notes(version, commits).await,
            ReleaseNotesFormat::Html => self.format_html_release_notes(version, commits).await,
            ReleaseNotesFormat::Custom(_) => self.format_custom_release_notes(version, commits).await,
        }
    }

    /// Format markdown release notes
    async fn format_markdown_release_notes(&self, version: &str, commits: &[CommitInfo]) -> RhemaResult<String> {
        let mut release_notes = String::new();
        
        // Header
        release_notes.push_str(&format!("# Release Notes - {}\n\n", version));
        release_notes.push_str(&format!("**Release Date:** {}\n\n", Utc::now().format("%Y-%m-%d")));
        
        // Breaking changes section
        if self.config.release_notes.include_breaking_changes {
            let breaking_changes: Vec<&CommitInfo> = commits.iter().filter(|c| c.breaking_change).collect();
            if !breaking_changes.is_empty() {
                release_notes.push_str("## Breaking Changes\n\n");
                for commit in breaking_changes {
                    release_notes.push_str(&format!("- {}\n", commit.description));
                }
                release_notes.push('\n');
            }
        }
        
        // Features section
        let features: Vec<&CommitInfo> = commits.iter().filter(|c| matches!(c.commit_type, CommitType::Feature)).collect();
        if !features.is_empty() {
            release_notes.push_str("## New Features\n\n");
            for commit in features {
                release_notes.push_str(&format!("- {}\n", commit.description));
            }
            release_notes.push('\n');
        }
        
        // Bug fixes section
        let fixes: Vec<&CommitInfo> = commits.iter().filter(|c| matches!(c.commit_type, CommitType::Fix)).collect();
        if !fixes.is_empty() {
            release_notes.push_str("## Bug Fixes\n\n");
            for commit in fixes {
                release_notes.push_str(&format!("- {}\n", commit.description));
            }
            release_notes.push('\n');
        }
        
        // Other changes section
        let other_changes: Vec<&CommitInfo> = commits.iter()
            .filter(|c| !matches!(c.commit_type, CommitType::Feature | CommitType::Fix) && !c.breaking_change)
            .collect();
        if !other_changes.is_empty() {
            release_notes.push_str("## Other Changes\n\n");
            for commit in other_changes {
                release_notes.push_str(&format!("- {}\n", commit.description));
            }
            release_notes.push('\n');
        }
        
        // Migration guide section
        if self.config.release_notes.include_migration_guide {
            let breaking_changes: Vec<&CommitInfo> = commits.iter().filter(|c| c.breaking_change).collect();
            if !breaking_changes.is_empty() {
                release_notes.push_str("## Migration Guide\n\n");
                release_notes.push_str("This release includes breaking changes. Please review the following changes:\n\n");
                for commit in breaking_changes {
                    release_notes.push_str(&format!("- {}\n", commit.description));
                }
                release_notes.push('\n');
            }
        }
        
        // Security notes section
        if self.config.release_notes.include_security_notes {
            let security_commits: Vec<&CommitInfo> = commits.iter()
                .filter(|c| c.description.to_lowercase().contains("security") || 
                           c.description.to_lowercase().contains("vulnerability") ||
                           c.description.to_lowercase().contains("cve"))
                .collect();
            if !security_commits.is_empty() {
                release_notes.push_str("## Security Notes\n\n");
                for commit in security_commits {
                    release_notes.push_str(&format!("- {}\n", commit.description));
                }
                release_notes.push('\n');
            }
        }
        
        Ok(release_notes)
    }

    /// Format HTML release notes
    async fn format_html_release_notes(&self, version: &str, commits: &[CommitInfo]) -> RhemaResult<String> {
        let markdown = self.format_markdown_release_notes(version, commits).await?;
        
        // Simple markdown to HTML conversion
        let html = markdown
            .replace("# ", "<h1>")
            .replace("\n\n", "</h1>\n\n")
            .replace("## ", "<h2>")
            .replace("\n\n", "</h2>\n\n")
            .replace("- ", "<li>")
            .replace("\n", "</li>\n");
        
        let html = format!("<!DOCTYPE html>\n<html>\n<head>\n<title>Release Notes - {}</title>\n</head>\n<body>\n{}\n</body>\n</html>", version, html);
        
        Ok(html)
    }

    /// Format custom release notes
    async fn format_custom_release_notes(&self, version: &str, commits: &[CommitInfo]) -> RhemaResult<String> {
        // Default to markdown format for custom
        self.format_markdown_release_notes(version, commits).await
    }
}

/// Default version management configuration
pub fn default_version_management_config() -> VersionManagementConfig {
    VersionManagementConfig {
        strategy: VersioningStrategy::Semantic,
        version_files: vec![
            VersionFile {
                path: PathBuf::from("Cargo.toml"),
                file_type: VersionFileType::CargoToml,
                version_pattern: r#"version\s*=\s*"([^"]+)""#.to_string(),
                replacement_pattern: r#"version = "{}""#.to_string(),
                required: true,
            },
        ],
        changelog: ChangelogConfig {
            file_path: PathBuf::from("CHANGELOG.md"),
            format: ChangelogFormat::Markdown,
            include_commit_hashes: true,
            include_author: false,
            include_date: true,
            group_by_type: true,
            commit_types: HashMap::new(),
            template: None,
        },
        release_notes: ReleaseNotesConfig {
            directory: PathBuf::from("release-notes"),
            format: ReleaseNotesFormat::Markdown,
            include_breaking_changes: true,
            include_migration_guide: true,
            include_security_notes: true,
            template: None,
        },
        commit_patterns: CommitPatterns {
            major_bump: vec![
                r"!breaking".to_string(),
                r"BREAKING CHANGE".to_string(),
                r"major".to_string(),
            ],
            minor_bump: vec![
                r"feat".to_string(),
                r"feature".to_string(),
                r"minor".to_string(),
            ],
            patch_bump: vec![
                r"fix".to_string(),
                r"bugfix".to_string(),
                r"patch".to_string(),
            ],
            breaking_change: vec![
                r"!breaking".to_string(),
                r"BREAKING CHANGE".to_string(),
            ],
            ignore: vec![
                r"docs".to_string(),
                r"style".to_string(),
                r"chore".to_string(),
            ],
        },
        auto_bump: AutoBumpConfig {
            enabled: true,
            strategy: BumpStrategy::Conservative,
            analyze_commits: true,
            analyze_changes: false,
            min_confidence: 0.8,
            confirm_major_bumps: true,
        },
        validation: VersionValidationConfig {
            validate_semver: true,
            require_consistency: true,
            validate_format: true,
            custom_rules: Vec::new(),
        },
    }
} 