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
use git2::{BlameOptions, Commit, DiffOptions, Repository};
use rhema_core::{RhemaError, RhemaResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// These types are defined in this same module, so we don't need to import them
// Remove the self-import that was causing duplicate definitions

/// Enhanced context evolution entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextEvolution {
    /// Commit hash
    pub commit_hash: String,

    /// Commit message
    pub commit_message: String,

    /// Author information
    pub author: AuthorInfo,

    /// Timestamp
    pub timestamp: DateTime<Utc>,

    /// Context changes
    pub changes: Vec<ContextChange>,

    /// Impact analysis
    pub impact: Option<ImpactAnalysis>,

    /// Related knowledge entries
    pub related_knowledge: Vec<String>,

    /// Tags and labels
    pub tags: Vec<String>,

    /// Advanced evolution features
    pub advanced_features: AdvancedEvolutionFeatures,

    /// Context analytics
    pub analytics: ContextAnalytics,

    /// Context metadata
    pub metadata: ContextMetadata,
}

/// Author information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorInfo {
    pub name: String,
    pub email: String,
    pub timestamp: DateTime<Utc>,
}

impl std::fmt::Display for AuthorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

/// Context change details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextChange {
    /// File path
    pub file_path: PathBuf,

    /// Change type
    pub change_type: ChangeType,

    /// Change description
    pub description: String,

    /// Lines added
    pub lines_added: Option<usize>,

    /// Lines removed
    pub lines_removed: Option<usize>,

    /// Specific changes
    pub specific_changes: Vec<SpecificChange>,

    /// Impact level
    pub impact_level: ImpactLevel,

    /// Related scopes
    pub related_scopes: Vec<String>,
}

/// Change type
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
    Moved,
}

/// Specific change details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecificChange {
    /// Field or section changed
    pub field: String,

    /// Old value
    pub old_value: Option<String>,

    /// New value
    pub new_value: Option<String>,

    /// Change description
    pub description: String,
}

/// Impact level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Impact analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysis {
    /// Affected scopes
    pub affected_scopes: Vec<String>,

    /// Affected dependencies
    pub affected_dependencies: Vec<String>,

    /// Breaking changes
    pub breaking_changes: Vec<String>,

    /// Risk assessment
    pub risk_assessment: RiskAssessment,

    /// Recommendations
    pub recommendations: Vec<String>,

    /// Performance impact
    pub performance_impact: String,

    /// Dependency impact
    pub dependency_impact: String,

    /// Test impact
    pub test_impact: String,

    /// Documentation impact
    pub documentation_impact: String,

    /// Security impact
    pub security_impact: String,

    /// Risk level
    pub risk_level: String,
}

/// Risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk level
    pub risk_level: RiskLevel,

    /// Risk factors
    pub risk_factors: Vec<RiskFactor>,

    /// Mitigation strategies
    pub mitigations: Vec<String>,
}

/// Risk level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Risk factor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor: String,
    pub description: String,
    pub severity: RiskLevel,
}

/// Git blame information for context entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextBlame {
    /// File path
    pub file_path: PathBuf,

    /// Line number
    pub line_number: usize,

    /// Commit hash
    pub commit_hash: String,

    /// Author information
    pub author: AuthorInfo,

    /// Line content
    pub content: String,

    /// Context entry type
    pub entry_type: Option<String>,

    /// Entry identifier
    pub entry_id: Option<String>,

    /// Advanced blame features
    pub advanced_features: AdvancedBlameFeatures,

    /// Blame analytics
    pub analytics: BlameAnalytics,
}

/// Enhanced context version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextVersion {
    /// Version identifier
    pub version: String,

    /// Commit hash
    pub commit_hash: String,

    /// Version type
    pub version_type: VersionType,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Description
    pub description: String,

    /// Context snapshot
    pub snapshot: ContextSnapshot,

    /// Tags
    pub tags: Vec<String>,

    /// Advanced version features
    pub advanced_features: AdvancedVersionFeatures,

    /// Version analytics
    pub analytics: VersionAnalytics,

    /// Version metadata
    pub metadata: VersionMetadata,
}

/// Version type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionType {
    Major,
    Minor,
    Patch,
    PreRelease,
    Custom(String),
}

impl std::fmt::Display for VersionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionType::Major => write!(f, "major"),
            VersionType::Minor => write!(f, "minor"),
            VersionType::Patch => write!(f, "patch"),
            VersionType::PreRelease => write!(f, "prerelease"),
            VersionType::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Context snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSnapshot {
    /// Scope definitions
    pub scopes: HashMap<String, String>,

    /// Knowledge entries
    pub knowledge: HashMap<String, String>,

    /// Todo items
    pub todos: HashMap<String, String>,

    /// Decisions
    pub decisions: HashMap<String, String>,

    /// Patterns
    pub patterns: HashMap<String, String>,

    /// Conventions
    pub conventions: HashMap<String, String>,

    /// Files
    pub files: HashMap<String, String>,

    /// Context files
    pub context_files: HashMap<String, String>,

    /// Timestamp
    pub timestamp: Option<DateTime<Utc>>,

    /// ID
    pub id: String,
}

/// Advanced evolution features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedEvolutionFeatures {
    /// Evolution type
    pub evolution_type: EvolutionType,

    /// Evolution category
    pub evolution_category: EvolutionCategory,

    /// Evolution priority
    pub evolution_priority: EvolutionPriority,

    /// Evolution complexity
    pub evolution_complexity: EvolutionComplexity,

    /// Evolution risk level
    pub evolution_risk: EvolutionRisk,

    /// Evolution dependencies
    pub evolution_dependencies: Vec<String>,

    /// Evolution blockers
    pub evolution_blockers: Vec<String>,

    /// Evolution approvals
    pub evolution_approvals: Vec<Approval>,
}

/// Evolution type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionType {
    Feature,
    BugFix,
    Refactor,
    Documentation,
    Configuration,
    Security,
    Performance,
    Breaking,
    Custom(String),
}

/// Evolution category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionCategory {
    Core,
    Extension,
    Integration,
    Utility,
    Test,
    Example,
    Custom(String),
}

/// Evolution priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionPriority {
    Critical,
    High,
    Medium,
    Low,
    Optional,
}

/// Evolution complexity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionComplexity {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

/// Evolution risk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionRisk {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Approval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub approver: String,
    pub timestamp: DateTime<Utc>,
    pub status: ApprovalStatus,
    pub comments: Option<String>,
}

/// Approval status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Conditional,
}

/// Context analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAnalytics {
    /// Change frequency
    pub change_frequency: f64,

    /// Change velocity
    pub change_velocity: f64,

    /// Change impact score
    pub impact_score: f64,

    /// Change complexity score
    pub complexity_score: f64,

    /// Change risk score
    pub risk_score: f64,

    /// Change quality score
    pub quality_score: f64,

    /// Change maturity score
    pub maturity_score: f64,

    /// Change stability score
    pub stability_score: f64,
}

/// Context metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMetadata {
    /// Context version
    pub version: String,

    /// Context schema version
    pub schema_version: String,

    /// Context checksum
    pub checksum: String,

    /// Context size
    pub size: u64,

    /// Context compression ratio
    pub compression_ratio: Option<f64>,

    /// Context encryption status
    pub encrypted: bool,

    /// Context backup status
    pub backed_up: bool,

    /// Context validation status
    pub validation_status: ValidationStatus,

    /// Context dependencies
    pub dependencies: Vec<String>,

    /// Context references
    pub references: Vec<String>,
}

/// Enhanced blame features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedBlameFeatures {
    /// Blame type
    pub blame_type: BlameType,

    /// Blame category
    pub blame_category: BlameCategory,

    /// Blame severity
    pub blame_severity: BlameSeverity,

    /// Blame impact
    pub blame_impact: BlameImpact,

    /// Blame context
    pub blame_context: BlameContext,

    /// Blame history
    pub blame_history: Vec<BlameHistoryEntry>,
}

/// Blame type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlameType {
    Addition,
    Modification,
    Deletion,
    Movement,
    Refactor,
    Custom(String),
}

/// Blame category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlameCategory {
    Feature,
    BugFix,
    Documentation,
    Configuration,
    Security,
    Performance,
    Custom(String),
}

/// Blame severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlameSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Blame impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlameImpact {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Blame context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameContext {
    /// Related files
    pub related_files: Vec<PathBuf>,

    /// Related commits
    pub related_commits: Vec<String>,

    /// Related issues
    pub related_issues: Vec<String>,

    /// Related pull requests
    pub related_prs: Vec<String>,

    /// Context scope
    pub context_scope: String,

    /// Context domain
    pub context_domain: String,
}

/// Blame history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameHistoryEntry {
    pub commit_hash: String,
    pub timestamp: DateTime<Utc>,
    pub author: String,
    pub change_type: String,
    pub description: String,
}

/// Blame analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlameAnalytics {
    /// Blame frequency
    pub blame_frequency: f64,

    /// Blame velocity
    pub blame_velocity: f64,

    /// Blame impact score
    pub impact_score: f64,

    /// Blame complexity score
    pub complexity_score: f64,

    /// Blame risk score
    pub risk_score: f64,

    /// Blame quality score
    pub quality_score: f64,
}

/// Advanced version features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedVersionFeatures {
    /// Version stability
    pub stability: VersionStability,

    /// Version maturity
    pub maturity: VersionMaturity,

    /// Version compatibility
    pub compatibility: VersionCompatibility,

    /// Version dependencies
    pub dependencies: Vec<VersionDependency>,

    /// Version breaking changes
    pub breaking_changes: Vec<BreakingChange>,

    /// Version deprecations
    pub deprecations: Vec<Deprecation>,

    /// Version migrations
    pub migrations: Vec<Migration>,
}

/// Version stability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionStability {
    Stable,
    Beta,
    Alpha,
    Experimental,
    Deprecated,
}

/// Version maturity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionMaturity {
    Immature,
    Growing,
    Mature,
    Declining,
    Legacy,
}

/// Version compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionCompatibility {
    pub backward_compatible: bool,
    pub forward_compatible: bool,
    pub compatible_versions: Vec<String>,
    pub incompatible_versions: Vec<String>,
    pub migration_required: bool,
}

/// Version dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDependency {
    pub name: String,
    pub version: String,
    pub type_: DependencyType,
    pub required: bool,
    pub description: String,
}

/// Dependency type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Required,
    Optional,
    Development,
    Test,
    Custom(String),
}

/// Breaking change
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    pub description: String,
    pub impact: BreakingChangeImpact,
    pub migration_guide: Option<String>,
    pub affected_components: Vec<String>,
}

/// Breaking change impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BreakingChangeImpact {
    Low,
    Medium,
    High,
    Critical,
}

/// Deprecation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deprecation {
    pub description: String,
    pub replacement: Option<String>,
    pub removal_version: Option<String>,
    pub migration_guide: Option<String>,
}

/// Migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    pub from_version: String,
    pub to_version: String,
    pub description: String,
    pub steps: Vec<MigrationStep>,
    pub automated: bool,
}

/// Migration step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    pub step_number: u32,
    pub description: String,
    pub command: Option<String>,
    pub manual: bool,
    pub rollback: Option<String>,
}

/// Version analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAnalytics {
    /// Version adoption rate
    pub adoption_rate: f64,

    /// Version stability score
    pub stability_score: f64,

    /// Version maturity score
    pub maturity_score: f64,

    /// Version quality score
    pub quality_score: f64,

    /// Version performance score
    pub performance_score: f64,

    /// Version security score
    pub security_score: f64,
}

/// Version metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionMetadata {
    /// Version checksum
    pub checksum: String,

    /// Version size
    pub size: u64,

    /// Version compression ratio
    pub compression_ratio: Option<f64>,

    /// Version encryption status
    pub encrypted: bool,

    /// Version backup status
    pub backed_up: bool,

    /// Version validation status
    pub validation_status: ValidationStatus,

    /// Version signatures
    pub signatures: Vec<Signature>,

    /// Version certificates
    pub certificates: Vec<Certificate>,
}

/// Signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub signer: String,
    pub algorithm: String,
    pub signature: String,
    pub timestamp: DateTime<Utc>,
    pub verified: bool,
}

/// Certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub issuer: String,
    pub subject: String,
    pub serial_number: String,
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub certificate: String,
}

/// Automated commit message configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMessageConfig {
    /// Enable automated commit messages
    pub enabled: bool,

    /// Commit message template
    pub template: String,

    /// Commit message format
    pub format: CommitMessageFormat,

    /// Commit message rules
    pub rules: Vec<CommitMessageRule>,

    /// Commit message validation
    pub validation: CommitMessageValidation,

    /// Commit message automation
    pub automation: CommitMessageAutomation,
}

/// Commit message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommitMessageFormat {
    Conventional,
    Semantic,
    Custom(String),
}

/// Commit message rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMessageRule {
    pub name: String,
    pub description: String,
    pub pattern: String,
    pub required: bool,
    pub severity: ValidationSeverity,
}

/// Commit message validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMessageValidation {
    /// Validate commit message format
    pub validate_format: bool,

    /// Validate commit message length
    pub validate_length: bool,

    /// Validate commit message content
    pub validate_content: bool,

    /// Validate commit message references
    pub validate_references: bool,

    /// Maximum commit message length
    pub max_length: Option<usize>,

    /// Minimum commit message length
    pub min_length: Option<usize>,
}

/// Commit message automation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMessageAutomation {
    /// Auto-generate commit messages
    pub auto_generate: bool,

    /// Auto-format commit messages
    pub auto_format: bool,

    /// Auto-validate commit messages
    pub auto_validate: bool,

    /// Auto-correct commit messages
    pub auto_correct: bool,

    /// Auto-suggest commit messages
    pub auto_suggest: bool,
}

/// Validation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Valid,
    Invalid(Vec<String>),
    Pending,
    Skipped,
    Unknown,
}

/// Validation severity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
    Critical,
    High,
    Medium,
}

/// Context history manager
pub struct ContextHistoryManager {
    repo: Repository,
    evolution_cache: HashMap<String, Vec<ContextEvolution>>,
    blame_cache: HashMap<PathBuf, Vec<ContextBlame>>,
    version_cache: HashMap<String, ContextVersion>,
}

impl ContextHistoryManager {
    /// Create a new context history manager
    pub fn new(repo: Repository) -> Self {
        Self {
            repo,
            evolution_cache: HashMap::new(),
            blame_cache: HashMap::new(),
            version_cache: HashMap::new(),
        }
    }

    /// Track context evolution for a specific scope
    pub fn track_context_evolution(
        &mut self,
        scope_path: &str,
        limit: Option<usize>,
    ) -> RhemaResult<Vec<ContextEvolution>> {
        let cache_key = scope_path.to_string();

        if let Some(cached) = self.evolution_cache.get(&cache_key) {
            return Ok(cached.clone());
        }

        let mut evolution = Vec::new();
        let mut revwalk = self.repo.revwalk()?;

        // Add HEAD to start walking from the latest commit
        revwalk.push_head()?;

        let limit = limit.unwrap_or(100);
        let mut count = 0;

        for oid in revwalk {
            if count >= limit {
                break;
            }

            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            // Check if this commit affects the scope
            if self.commit_affects_scope(&commit, scope_path)? {
                let evolution_entry = self.create_evolution_entry(&commit, scope_path)?;
                evolution.push(evolution_entry);
                count += 1;
            }
        }

        self.evolution_cache.insert(cache_key, evolution.clone());
        Ok(evolution)
    }

    /// Check if a commit affects a specific scope
    fn commit_affects_scope(&self, commit: &Commit, scope_path: &str) -> RhemaResult<bool> {
        let tree = commit.tree()?;
        let parent_tree = if let Ok(parent) = commit.parent(0) {
            parent.tree().ok()
        } else {
            None
        };

        let mut diff_options = DiffOptions::new();
        let diff = if let Some(parent_tree) = parent_tree {
            self.repo
                .diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_options))?
        } else {
            self.repo
                .diff_tree_to_tree(None, Some(&tree), Some(&mut diff_options))?
        };

        for delta in diff.deltas() {
            if let Some(new_file) = delta.new_file().path() {
                let file_path = new_file.to_string_lossy();

                // Check if file affects the scope
                if file_path.contains(scope_path) {
                    return Ok(true);
                }

                // Special case for root scope (".") - include context files
                if scope_path == "."
                    && (file_path.contains("context/")
                        || file_path.ends_with(".yaml")
                        || file_path.ends_with(".yml"))
                {
                    return Ok(true);
                }
            }
            if let Some(old_file) = delta.old_file().path() {
                let file_path = old_file.to_string_lossy();

                // Check if file affects the scope
                if file_path.contains(scope_path) {
                    return Ok(true);
                }

                // Special case for root scope (".") - include context files
                if scope_path == "."
                    && (file_path.contains("context/")
                        || file_path.ends_with(".yaml")
                        || file_path.ends_with(".yml"))
                {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Create evolution entry from commit
    fn create_evolution_entry(
        &self,
        commit: &Commit,
        scope_path: &str,
    ) -> RhemaResult<ContextEvolution> {
        let changes = self.analyze_commit_changes(commit, scope_path)?;
        let impact = self.analyze_commit_impact(commit, scope_path)?;

        Ok(ContextEvolution {
            commit_hash: commit.id().to_string(),
            commit_message: commit.message().unwrap_or("").to_string(),
            author: AuthorInfo {
                name: commit.author().name().unwrap_or("").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
                timestamp: DateTime::from_timestamp(commit.author().when().seconds(), 0)
                    .unwrap_or_else(|| Utc::now()),
            },
            timestamp: DateTime::from_timestamp(commit.time().seconds(), 0)
                .unwrap_or_else(|| Utc::now()),
            changes,
            impact,
            related_knowledge: Vec::new(),
            tags: Vec::new(),
            advanced_features: AdvancedEvolutionFeatures {
                evolution_type: EvolutionType::Custom("Unknown".to_string()),
                evolution_category: EvolutionCategory::Custom("Unknown".to_string()),
                evolution_priority: EvolutionPriority::Optional,
                evolution_complexity: EvolutionComplexity::Simple,
                evolution_risk: EvolutionRisk::None,
                evolution_dependencies: Vec::new(),
                evolution_blockers: Vec::new(),
                evolution_approvals: Vec::new(),
            },
            analytics: ContextAnalytics {
                change_frequency: 0.0,
                change_velocity: 0.0,
                impact_score: 0.0,
                complexity_score: 0.0,
                risk_score: 0.0,
                quality_score: 0.0,
                maturity_score: 0.0,
                stability_score: 0.0,
            },
            metadata: ContextMetadata {
                version: "0.0.0".to_string(),
                schema_version: "1.0.0".to_string(),
                checksum: "".to_string(),
                size: 0,
                compression_ratio: None,
                encrypted: false,
                backed_up: false,
                validation_status: ValidationStatus::Unknown,
                dependencies: Vec::new(),
                references: Vec::new(),
            },
        })
    }

    /// Analyze changes in a commit
    fn analyze_commit_changes(
        &self,
        commit: &Commit,
        scope_path: &str,
    ) -> RhemaResult<Vec<ContextChange>> {
        let mut changes = Vec::new();
        let tree = commit.tree()?;
        let parent_tree = if let Ok(parent) = commit.parent(0) {
            parent.tree().ok()
        } else {
            None
        };

        let mut diff_options = DiffOptions::new();
        let diff = if let Some(parent_tree) = parent_tree {
            self.repo
                .diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_options))?
        } else {
            self.repo
                .diff_tree_to_tree(None, Some(&tree), Some(&mut diff_options))?
        };

        for delta in diff.deltas() {
            if let Some(new_file) = delta.new_file().path() {
                if new_file.to_string_lossy().contains(scope_path) {
                    let change = self.create_context_change(delta, new_file)?;
                    changes.push(change);
                }
            }
        }

        Ok(changes)
    }

    /// Create context change from delta
    fn create_context_change(
        &self,
        delta: git2::DiffDelta,
        file_path: &Path,
    ) -> RhemaResult<ContextChange> {
        let change_type = match delta.status() {
            git2::Delta::Added => ChangeType::Added,
            git2::Delta::Modified => ChangeType::Modified,
            git2::Delta::Deleted => ChangeType::Deleted,
            git2::Delta::Renamed => ChangeType::Renamed,
            _ => ChangeType::Modified,
        };

        let (lines_added, lines_removed) = self.calculate_line_changes(&delta)?;
        let specific_changes = self.analyze_specific_changes(&delta)?;

        Ok(ContextChange {
            file_path: file_path.to_path_buf(),
            change_type: change_type.clone(),
            description: format!("{:?} changes to {}", change_type, file_path.display()),
            lines_added,
            lines_removed,
            specific_changes,
            impact_level: self.assess_impact_level(&delta),
            related_scopes: self.extract_related_scopes(file_path),
        })
    }

    /// Calculate line changes
    fn calculate_line_changes(
        &self,
        delta: &git2::DiffDelta,
    ) -> RhemaResult<(Option<usize>, Option<usize>)> {
        // Implement line change calculation
        // This would analyze the actual diff to count added/removed lines

        if let Some(new_file) = delta.new_file().path() {
            if let Some(old_file) = delta.old_file().path() {
                // For now, we'll skip the diff analysis since the API is complex
                // In a real implementation, you would use the correct diff_blobs API
                let added_lines = 0;
                let removed_lines = 0;

                return Ok((Some(added_lines), Some(removed_lines)));
            }
        }

        // Fallback: estimate based on file size changes
        let old_size = delta.old_file().size();
        let new_size = delta.new_file().size();

        if old_size > 0 && new_size > 0 {
            let size_diff = if new_size > old_size {
                new_size - old_size
            } else {
                old_size - new_size
            };

            // Rough estimate: assume average line is 80 characters
            let estimated_lines = size_diff / 80;

            if new_size > old_size {
                Ok((Some(estimated_lines as usize), None))
            } else {
                Ok((None, Some(estimated_lines as usize)))
            }
        } else {
            Ok((None, None))
        }
    }

    /// Analyze specific changes
    fn analyze_specific_changes(
        &self,
        delta: &git2::DiffDelta,
    ) -> RhemaResult<Vec<SpecificChange>> {
        // Implement specific change analysis
        // This would parse the diff to identify specific field changes

        let changes = Vec::new();

        if let Some(new_file) = delta.new_file().path() {
            if let Some(old_file) = delta.old_file().path() {
                // For now, we'll skip the diff analysis since the API is complex
                // In a real implementation, you would use the correct diff_blobs API
                // and analyze the content for specific changes
            }
        }

        Ok(changes)
    }

    fn analyze_line_content(
        &self,
        content: &str,
        line: git2::DiffLine,
    ) -> RhemaResult<Option<SpecificChange>> {
        // Analyze a single line for specific changes

        // Check for common patterns
        if content.contains("version") {
            return Ok(Some(SpecificChange {
                field: "version".to_string(),
                old_value: None, // Would need to extract from diff
                new_value: Some(content.trim().to_string()),
                description: "Version update".to_string(),
            }));
        }

        if content.contains("dependency")
            || content.contains("Cargo.toml")
            || content.contains("package.json")
        {
            return Ok(Some(SpecificChange {
                field: "dependencies".to_string(),
                old_value: None,
                new_value: Some(content.trim().to_string()),
                description: "Dependency update".to_string(),
            }));
        }

        if content.contains("TODO") || content.contains("FIXME") {
            return Ok(Some(SpecificChange {
                field: "todos".to_string(),
                old_value: None,
                new_value: Some(content.trim().to_string()),
                description: "Todo update".to_string(),
            }));
        }

        if content.contains("test") || content.contains("spec") {
            return Ok(Some(SpecificChange {
                field: "tests".to_string(),
                old_value: None,
                new_value: Some(content.trim().to_string()),
                description: "Test update".to_string(),
            }));
        }

        // Check for configuration changes
        if content.contains("config") || content.contains("setting") {
            return Ok(Some(SpecificChange {
                field: "configuration".to_string(),
                old_value: None,
                new_value: Some(content.trim().to_string()),
                description: "Config update".to_string(),
            }));
        }

        Ok(None)
    }

    /// Assess impact level
    fn assess_impact_level(&self, delta: &git2::DiffDelta) -> ImpactLevel {
        // Implement impact level assessment
        // This would analyze the type and scope of changes

        // Check file type and location
        if let Some(file_path) = delta.new_file().path() {
            let path_str = file_path.to_string_lossy();

            // High impact files
            if path_str.contains("Cargo.toml") || path_str.contains("package.json") {
                return ImpactLevel::High; // Dependency changes
            }

            if path_str.contains("src/main") || path_str.contains("src/lib") {
                return ImpactLevel::High; // Core source files
            }

            if path_str.contains("tests/") || path_str.contains("__tests__") {
                return ImpactLevel::Medium; // Test files
            }

            if path_str.contains("docs/") || path_str.contains("README") {
                return ImpactLevel::Low; // Documentation
            }

            if path_str.contains("config") || path_str.contains(".env") {
                return ImpactLevel::Medium; // Configuration files
            }

            // Check file size changes
            let old_size = delta.old_file().size();
            let new_size = delta.new_file().size();

            if old_size > 0 && new_size > 0 {
                let size_change = if new_size > old_size {
                    new_size - old_size
                } else {
                    old_size - new_size
                };

                let change_percentage = (size_change as f64 / old_size as f64) * 100.0;

                if change_percentage > 50.0 {
                    return ImpactLevel::High; // Major changes
                } else if change_percentage > 20.0 {
                    return ImpactLevel::Medium; // Moderate changes
                } else {
                    return ImpactLevel::Low; // Minor changes
                }
            }
        }

        // Check file status
        match delta.status() {
            git2::Delta::Added => ImpactLevel::Medium,    // New files
            git2::Delta::Deleted => ImpactLevel::High,    // Deleted files
            git2::Delta::Modified => ImpactLevel::Medium, // Modified files
            git2::Delta::Renamed => ImpactLevel::Low,     // Renamed files
            _ => ImpactLevel::Low,                        // Other changes
        }
    }

    /// Extract related scopes
    fn extract_related_scopes(&self, file_path: &Path) -> Vec<String> {
        // Implement scope extraction
        // This would analyze the file path and content to identify related scopes

        let mut scopes = Vec::new();
        let path_str = file_path.to_string_lossy();

        // Extract scope from file path
        if path_str.contains("src/") {
            scopes.push("source".to_string());

            // Extract module scope
            if let Some(module_path) = path_str.split("src/").nth(1) {
                if let Some(module) = module_path.split('/').next() {
                    scopes.push(format!("module:{}", module));
                }
            }
        }

        if path_str.contains("tests/") {
            scopes.push("testing".to_string());
        }

        if path_str.contains("docs/") {
            scopes.push("documentation".to_string());
        }

        if path_str.contains("examples/") {
            scopes.push("examples".to_string());
        }

        if path_str.contains("config/") || path_str.contains(".config") {
            scopes.push("configuration".to_string());
        }

        if path_str.contains("scripts/") {
            scopes.push("scripts".to_string());
        }

        // Extract language scope
        if let Some(extension) = file_path.extension() {
            match extension.to_string_lossy().as_ref() {
                "rs" => scopes.push("rust".to_string()),
                "js" | "ts" => scopes.push("javascript".to_string()),
                "py" => scopes.push("python".to_string()),
                "go" => scopes.push("go".to_string()),
                "java" => scopes.push("java".to_string()),
                "cpp" | "cc" | "cxx" => scopes.push("cpp".to_string()),
                "c" => scopes.push("c".to_string()),
                "md" | "mdx" => scopes.push("markdown".to_string()),
                "yaml" | "yml" => scopes.push("yaml".to_string()),
                "json" => scopes.push("json".to_string()),
                "toml" => scopes.push("toml".to_string()),
                "sh" | "bash" => scopes.push("shell".to_string()),
                _ => scopes.push("unknown".to_string()),
            }
        }

        // Extract feature scope
        if let Some(parent_dir) = file_path.parent() {
            let parent_path_str = parent_dir.to_string_lossy();
            if parent_path_str.contains("feature/") {
                scopes.push("feature".to_string());
            }
            if parent_path_str.contains("bugfix/") {
                scopes.push("bugfix".to_string());
            }
            if parent_path_str.contains("hotfix/") {
                scopes.push("hotfix".to_string());
            }
            if parent_path_str.contains("release/") {
                scopes.push("release".to_string());
            }
        }

        // Extract domain scope
        if let Some(parent_dir) = file_path.parent() {
            let parent_path_str = parent_dir.to_string_lossy();
            if parent_path_str.contains("api/") {
                scopes.push("api".to_string());
            }
            if parent_path_str.contains("ui/") || parent_path_str.contains("frontend/") {
                scopes.push("ui".to_string());
            }
            if parent_path_str.contains("backend/") || parent_path_str.contains("server/") {
                scopes.push("backend".to_string());
            }
            if parent_path_str.contains("database/") || parent_path_str.contains("db/") {
                scopes.push("database".to_string());
            }
        }

        scopes
    }

    /// Analyze commit impact
    fn analyze_commit_impact(
        &self,
        commit: &Commit,
        scope_path: &str,
    ) -> RhemaResult<Option<ImpactAnalysis>> {
        // Implement impact analysis
        // This would analyze the broader impact of the commit

        let mut impact_analysis = ImpactAnalysis {
            risk_level: "low".to_string(),
            affected_scopes: Vec::new(),
            affected_dependencies: Vec::new(),
            breaking_changes: Vec::new(),
            risk_assessment: RiskAssessment {
                risk_level: RiskLevel::Low,
                risk_factors: Vec::new(),
                mitigations: Vec::new(),
            },
            recommendations: Vec::new(),
            performance_impact: "none".to_string(),
            security_impact: "none".to_string(),
            dependency_impact: "none".to_string(),
            test_impact: "none".to_string(),
            documentation_impact: "none".to_string(),
        };

        // Analyze commit message
        let message = commit.message().unwrap_or("");
        let message_lower = message.to_lowercase();

        // Check for breaking changes
        if message_lower.contains("breaking") || message_lower.contains("breaking change") {
            impact_analysis.risk_level = "high".to_string();
            impact_analysis
                .breaking_changes
                .push("Breaking change detected in commit message".to_string());
        }

        // Check for performance changes
        if message_lower.contains("performance") || message_lower.contains("optimization") {
            impact_analysis.performance_impact = "moderate".to_string();
        }

        // Check for security changes
        if message_lower.contains("security") || message_lower.contains("vulnerability") {
            impact_analysis.security_impact = "high".to_string();
            impact_analysis.risk_level = "high".to_string();
        }

        // Check for dependency changes
        if message_lower.contains("dependency") || message_lower.contains("update") {
            impact_analysis.dependency_impact = "moderate".to_string();
        }

        // Check for test changes
        if message_lower.contains("test") || message_lower.contains("spec") {
            impact_analysis.test_impact = "low".to_string();
        }

        // Check for documentation changes
        if message_lower.contains("doc") || message_lower.contains("readme") {
            impact_analysis.documentation_impact = "low".to_string();
        }

        // Analyze affected scopes based on scope path
        if !scope_path.is_empty() {
            impact_analysis.affected_scopes.push(scope_path.to_string());
        }

        // Analyze file changes
        if let Ok(parent) = commit.parent(0) {
            let diff =
                self.repo
                    .diff_tree_to_tree(Some(&parent.tree()?), Some(&commit.tree()?), None)?;

            for delta in diff.deltas() {
                if let Some(file_path) = delta.new_file().path() {
                    let scopes = self.extract_related_scopes(file_path);
                    impact_analysis.affected_scopes.extend(scopes);
                }
            }
        }

        // Determine overall risk level
        if impact_analysis.security_impact == "high" || !impact_analysis.breaking_changes.is_empty()
        {
            impact_analysis.risk_level = "high".to_string();
        } else if impact_analysis.performance_impact == "moderate"
            || impact_analysis.dependency_impact == "moderate"
        {
            impact_analysis.risk_level = "medium".to_string();
        }

        // Remove duplicates from affected scopes
        impact_analysis.affected_scopes.sort();
        impact_analysis.affected_scopes.dedup();

        Ok(Some(impact_analysis))
    }

    /// Get Git blame for context entries
    pub fn get_context_blame(&mut self, file_path: &Path) -> RhemaResult<Vec<ContextBlame>> {
        if let Some(cached) = self.blame_cache.get(file_path) {
            return Ok(cached.clone());
        }

        let mut blame_options = BlameOptions::new();
        let blame = self.repo.blame_file(file_path, Some(&mut blame_options))?;

        let mut context_blame = Vec::new();

        for hunk in blame.iter() {
            let commit = self.repo.find_commit(hunk.final_commit_id())?;
            let line_content =
                self.get_line_content(file_path, hunk.final_start_line().try_into().unwrap_or(0))?;

            let blame_entry = ContextBlame {
                file_path: file_path.to_path_buf(),
                line_number: hunk.final_start_line() as usize,
                commit_hash: commit.id().to_string(),
                author: AuthorInfo {
                    name: commit.author().name().unwrap_or("").to_string(),
                    email: commit.author().email().unwrap_or("").to_string(),
                    timestamp: DateTime::from_timestamp(commit.author().when().seconds(), 0)
                        .unwrap_or_else(|| Utc::now()),
                },
                content: line_content.clone(),
                entry_type: Some(self.extract_entry_type(&line_content)),
                entry_id: Some(self.extract_entry_id(&line_content)),
                advanced_features: AdvancedBlameFeatures {
                    blame_type: BlameType::Custom("Unknown".to_string()),
                    blame_category: BlameCategory::Custom("Unknown".to_string()),
                    blame_severity: BlameSeverity::Info,
                    blame_impact: BlameImpact::None,
                    blame_context: BlameContext {
                        related_files: Vec::new(),
                        related_commits: Vec::new(),
                        related_issues: Vec::new(),
                        related_prs: Vec::new(),
                        context_scope: "".to_string(),
                        context_domain: "".to_string(),
                    },
                    blame_history: Vec::new(),
                },
                analytics: BlameAnalytics {
                    blame_frequency: 0.0,
                    blame_velocity: 0.0,
                    impact_score: 0.0,
                    complexity_score: 0.0,
                    risk_score: 0.0,
                    quality_score: 0.0,
                },
            };

            context_blame.push(blame_entry);
        }

        self.blame_cache
            .insert(file_path.to_path_buf(), context_blame.clone());
        Ok(context_blame)
    }

    /// Get line content from file
    fn get_line_content(&self, file_path: &Path, line_number: u32) -> RhemaResult<String> {
        let content = std::fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();

        if line_number < lines.len() as u32 {
            Ok(lines[line_number as usize].to_string())
        } else {
            Ok(String::new())
        }
    }

    /// Extract entry type from content
    fn extract_entry_type(&self, content: &str) -> String {
        // Implement entry type extraction
        // This would analyze the content to determine the entry type

        let content_lower = content.to_lowercase();

        // Check for common patterns
        if content_lower.contains("todo") || content_lower.contains("fixme") {
            return "todo".to_string();
        }

        if content_lower.contains("function") || content_lower.contains("fn ") {
            return "function".to_string();
        }

        if content_lower.contains("struct") || content_lower.contains("class") {
            return "struct".to_string();
        }

        if content_lower.contains("enum") {
            return "enum".to_string();
        }

        if content_lower.contains("trait") || content_lower.contains("interface") {
            return "trait".to_string();
        }

        if content_lower.contains("impl") {
            return "implementation".to_string();
        }

        if content_lower.contains("mod ") || content_lower.contains("module") {
            return "module".to_string();
        }

        if content_lower.contains("use ") || content_lower.contains("import") {
            return "import".to_string();
        }

        if content_lower.contains("pub ") || content_lower.contains("public") {
            return "public".to_string();
        }

        if content_lower.contains("const ") || content_lower.contains("static") {
            return "constant".to_string();
        }

        if content_lower.contains("test") || content_lower.contains("spec") {
            return "test".to_string();
        }

        if content_lower.contains("comment")
            || content_lower.contains("//")
            || content_lower.contains("/*")
        {
            return "comment".to_string();
        }

        if content_lower.contains("config") || content_lower.contains("setting") {
            return "configuration".to_string();
        }

        if content_lower.contains("error") || content_lower.contains("panic") {
            return "error".to_string();
        }

        if content_lower.contains("log") || content_lower.contains("println") {
            return "logging".to_string();
        }

        // Default to unknown
        "unknown".to_string()
    }

    /// Extract entry ID from content
    fn extract_entry_id(&self, content: &str) -> String {
        // Implement entry ID extraction
        // This would extract a unique identifier from the content

        // Look for common ID patterns
        let patterns = vec![
            r#"id\s*[:=]\s*["']([^"']+)["']"#,   // id: "value" or id = "value"
            r#"name\s*[:=]\s*["']([^"']+)["']"#, // name: "value" or name = "value"
            r#"key\s*[:=]\s*["']([^"']+)["']"#,  // key: "value" or key = "value"
            r#"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)"#,  // function name
            r#"struct\s+([a-zA-Z_][a-zA-Z0-9_]*)"#, // struct name
            r#"enum\s+([a-zA-Z_][a-zA-Z0-9_]*)"#, // enum name
            r#"trait\s+([a-zA-Z_][a-zA-Z0-9_]*)"#, // trait name
            r#"mod\s+([a-zA-Z_][a-zA-Z0-9_]*)"#, // module name
            r#"const\s+([a-zA-Z_][a-zA-Z0-9_]*)"#, // constant name
            r#"static\s+([a-zA-Z_][a-zA-Z0-9_]*)"#, // static name
        ];

        for pattern in patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                if let Some(captures) = regex.captures(content) {
                    if let Some(id) = captures.get(1) {
                        return id.as_str().to_string();
                    }
                }
            }
        }

        // Fallback: generate a hash-based ID
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    /// Implement rollback logic
    fn rollback_to_snapshot(&self, snapshot_id: &str) -> RhemaResult<()> {
        // Implement rollback logic
        eprintln!("Rolling back to snapshot: {}", snapshot_id);

        // Find the snapshot
        let snapshot = self.find_snapshot(snapshot_id)?;

        // Validate snapshot
        if !self.validate_snapshot(&snapshot)? {
            return Err(RhemaError::ValidationError(
                "Snapshot validation failed".to_string(),
            ));
        }

        // Create backup of current state
        self.create_backup("before_rollback")?;

        // Restore files from snapshot
        for (file_path, content) in &snapshot.files {
            let full_path = self
                .repo
                .path()
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join(file_path);

            // Create directory if it doesn't exist
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Write file content
            std::fs::write(&full_path, content)?;
        }

        // Restore context files
        for (file_path, content) in &snapshot.context_files {
            let full_path = self
                .repo
                .path()
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join(file_path);

            // Create directory if it doesn't exist
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Write file content
            std::fs::write(&full_path, content)?;
        }

        // Restore todos
        for (file_path, content) in &snapshot.todos {
            let full_path = self
                .repo
                .path()
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join(file_path);

            // Create directory if it doesn't exist
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Write file content
            std::fs::write(&full_path, content)?;
        }

        eprintln!("Rollback completed successfully");
        Ok(())
    }

    fn find_snapshot(&self, snapshot_id: &str) -> RhemaResult<ContextSnapshot> {
        // Find snapshot by ID
        let snapshots_dir = self
            .repo
            .path()
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(".rhema")
            .join("snapshots");

        let snapshot_file = snapshots_dir.join(format!("{}.json", snapshot_id));

        if !snapshot_file.exists() {
            return Err(RhemaError::ValidationError(format!(
                "Snapshot {} not found",
                snapshot_id
            )));
        }

        let content = std::fs::read_to_string(snapshot_file)?;
        let snapshot: ContextSnapshot = serde_json::from_str(&content)?;

        Ok(snapshot)
    }

    fn validate_snapshot(&self, snapshot: &ContextSnapshot) -> RhemaResult<bool> {
        // Validate snapshot integrity
        if snapshot.timestamp.is_none() {
            return Ok(false);
        }

        if snapshot.files.is_empty() && snapshot.context_files.is_empty() {
            return Ok(false);
        }

        // Check if files still exist
        for file_path in snapshot.files.keys() {
            let full_path = self
                .repo
                .path()
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join(file_path);

            if !full_path.exists() {
                eprintln!("Warning: File {} no longer exists", file_path);
            }
        }

        Ok(true)
    }

    fn create_backup(&self, backup_name: &str) -> RhemaResult<()> {
        // Create backup of current state
        let backup_dir = self
            .repo
            .path()
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(".rhema")
            .join("backups");

        std::fs::create_dir_all(&backup_dir)?;

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_file = backup_dir.join(format!("{}_{}.json", backup_name, timestamp));

        // Create backup snapshot
        let backup_snapshot = self.create_current_snapshot()?;
        let backup_json = serde_json::to_string_pretty(&backup_snapshot)?;
        std::fs::write(&backup_file, backup_json)?;

        eprintln!("Backup created: {:?}", backup_file);
        Ok(())
    }

    fn create_current_snapshot(&self) -> RhemaResult<ContextSnapshot> {
        // Create snapshot of current state
        let mut snapshot = ContextSnapshot {
            id: format!("snapshot_{}", chrono::Utc::now().timestamp()),
            timestamp: Some(chrono::Utc::now()),
            scopes: HashMap::new(),
            knowledge: HashMap::new(),
            todos: HashMap::new(),
            decisions: HashMap::new(),
            patterns: HashMap::new(),
            conventions: HashMap::new(),
            files: HashMap::new(),
            context_files: HashMap::new(),
        };

        // Scan for files
        let repo_parent = self
            .repo
            .path()
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));

        self.scan_directory_for_files(repo_parent, &mut snapshot)?;

        Ok(snapshot)
    }

    fn scan_directory_for_files(
        &self,
        dir: &Path,
        snapshot: &mut ContextSnapshot,
    ) -> RhemaResult<()> {
        // Recursively scan directory for files
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if path.is_file() {
                        let relative_path = path.strip_prefix(
                            self.repo
                                .path()
                                .parent()
                                .unwrap_or_else(|| std::path::Path::new(".")),
                        )?;

                        if let Ok(content) = std::fs::read_to_string(&path) {
                            let relative_path_str = relative_path.to_string_lossy().to_string();

                            // Categorize files
                            if relative_path_str.ends_with(".yaml")
                                || relative_path_str.ends_with(".yml")
                            {
                                if relative_path_str.contains("context")
                                    || relative_path_str.contains("config")
                                {
                                    snapshot.context_files.insert(relative_path_str, content);
                                } else {
                                    snapshot.files.insert(relative_path_str, content);
                                }
                            } else if relative_path_str.contains("todo")
                                || relative_path_str.contains("TODO")
                            {
                                snapshot.todos.insert(relative_path_str, content);
                            } else {
                                snapshot.files.insert(relative_path_str, content);
                            }
                        }
                    } else if path.is_dir() {
                        // Skip .git and .rhema directories
                        if let Some(name) = path.file_name() {
                            if name == ".git" || name == ".rhema" {
                                continue;
                            }
                        }

                        self.scan_directory_for_files(&path, snapshot)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Implement file-specific change analysis
    fn analyze_file_changes(&self, file_path: &Path) -> RhemaResult<FileChangeAnalysis> {
        // Implement file-specific change analysis
        let mut analysis = FileChangeAnalysis {
            file_path: file_path.to_path_buf(),
            change_type: ChangeType::Modified,
            impact_level: ImpactLevel::Medium,
            affected_scopes: Vec::new(),
            complexity_changes: Vec::new(),
            security_implications: Vec::new(),
            performance_implications: Vec::new(),
            recommendations: Vec::new(),
        };

        // Analyze file type
        if let Some(extension) = file_path.extension() {
            match extension.to_string_lossy().as_ref() {
                "rs" => {
                    analysis.affected_scopes.push("rust".to_string());
                    analysis.affected_scopes.push("source".to_string());
                }
                "js" | "ts" => {
                    analysis.affected_scopes.push("javascript".to_string());
                    analysis.affected_scopes.push("source".to_string());
                }
                "py" => {
                    analysis.affected_scopes.push("python".to_string());
                    analysis.affected_scopes.push("source".to_string());
                }
                "yaml" | "yml" => {
                    analysis.affected_scopes.push("configuration".to_string());
                }
                "json" => {
                    analysis.affected_scopes.push("configuration".to_string());
                }
                "toml" => {
                    analysis.affected_scopes.push("configuration".to_string());
                }
                "md" | "mdx" => {
                    analysis.affected_scopes.push("documentation".to_string());
                }
                _ => {
                    analysis.affected_scopes.push("unknown".to_string());
                }
            }
        }

        // Analyze file location
        let path_str = file_path.to_string_lossy();
        if path_str.contains("src/") {
            analysis.impact_level = ImpactLevel::High;
            analysis.affected_scopes.push("source".to_string());
        }

        if path_str.contains("tests/") {
            analysis.impact_level = ImpactLevel::Low;
            analysis.affected_scopes.push("testing".to_string());
        }

        if path_str.contains("docs/") {
            analysis.impact_level = ImpactLevel::Low;
            analysis.affected_scopes.push("documentation".to_string());
        }

        if path_str.contains("config/") {
            analysis.impact_level = ImpactLevel::Medium;
            analysis.affected_scopes.push("configuration".to_string());
        }

        // Analyze file content if available
        if let Ok(content) = std::fs::read_to_string(file_path) {
            self.analyze_file_content(&content, &mut analysis);
        }

        // Generate recommendations
        self.generate_file_recommendations(&mut analysis);

        Ok(analysis)
    }

    fn analyze_file_content(&self, content: &str, analysis: &mut FileChangeAnalysis) {
        // Analyze file content for various implications

        // Check for complexity indicators
        if content.contains("TODO") || content.contains("FIXME") {
            analysis
                .complexity_changes
                .push("Contains TODO/FIXME items".to_string());
        }

        if content.contains("unsafe") {
            analysis
                .security_implications
                .push("Contains unsafe code".to_string());
        }

        if content.contains("panic!") || content.contains("unwrap()") {
            analysis
                .security_implications
                .push("Contains potential panic points".to_string());
        }

        if content.contains("loop") || content.contains("while") {
            analysis
                .performance_implications
                .push("Contains loops - check for performance".to_string());
        }

        if content.contains("clone()") || content.contains("copy()") {
            analysis
                .performance_implications
                .push("Contains cloning operations".to_string());
        }

        // Check for security patterns
        if content.contains("password") || content.contains("secret") || content.contains("key") {
            analysis
                .security_implications
                .push("Contains potential sensitive data".to_string());
        }

        if content.contains("eval") || content.contains("exec") {
            analysis
                .security_implications
                .push("Contains potentially dangerous code execution".to_string());
        }
    }

    fn generate_file_recommendations(&self, analysis: &mut FileChangeAnalysis) {
        // Generate recommendations based on analysis

        if !analysis.complexity_changes.is_empty() {
            analysis
                .recommendations
                .push("Consider addressing TODO/FIXME items".to_string());
        }

        if !analysis.security_implications.is_empty() {
            analysis
                .recommendations
                .push("Review security implications".to_string());
        }

        if !analysis.performance_implications.is_empty() {
            analysis
                .recommendations
                .push("Consider performance implications".to_string());
        }

        if analysis.impact_level == ImpactLevel::High {
            analysis
                .recommendations
                .push("High impact change - consider thorough testing".to_string());
        }

        if analysis.affected_scopes.contains(&"source".to_string()) {
            analysis
                .recommendations
                .push("Source code change - run tests".to_string());
        }

        if analysis
            .affected_scopes
            .contains(&"configuration".to_string())
        {
            analysis
                .recommendations
                .push("Configuration change - validate settings".to_string());
        }
    }

    /// Create context version
    pub fn create_context_version(
        &mut self,
        version: &str,
        version_type: VersionType,
        description: &str,
    ) -> RhemaResult<ContextVersion> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;

        let snapshot = self.create_context_snapshot()?;

        let context_version = ContextVersion {
            version: version.to_string(),
            commit_hash: commit.id().to_string(),
            version_type,
            created_at: Utc::now(),
            description: description.to_string(),
            snapshot,
            tags: Vec::new(),
            advanced_features: AdvancedVersionFeatures {
                stability: VersionStability::Stable,
                maturity: VersionMaturity::Mature,
                compatibility: VersionCompatibility {
                    backward_compatible: true,
                    forward_compatible: true,
                    compatible_versions: Vec::new(),
                    incompatible_versions: Vec::new(),
                    migration_required: false,
                },
                dependencies: Vec::new(),
                breaking_changes: Vec::new(),
                deprecations: Vec::new(),
                migrations: Vec::new(),
            },
            analytics: VersionAnalytics {
                adoption_rate: 0.0,
                stability_score: 0.0,
                maturity_score: 0.0,
                quality_score: 0.0,
                performance_score: 0.0,
                security_score: 0.0,
            },
            metadata: VersionMetadata {
                checksum: "".to_string(),
                size: 0,
                compression_ratio: None,
                encrypted: false,
                backed_up: false,
                validation_status: ValidationStatus::Unknown,
                signatures: Vec::new(),
                certificates: Vec::new(),
            },
        };

        self.version_cache
            .insert(version.to_string(), context_version.clone());

        // Create Git tag for the version
        self.create_version_tag(version, &commit)?;

        Ok(context_version)
    }

    /// Create context snapshot
    fn create_context_snapshot(&self) -> RhemaResult<ContextSnapshot> {
        let repo_path = self.repo.path().parent().ok_or_else(|| {
            RhemaError::GitError(git2::Error::from_str("Invalid repository path"))
        })?;

        let mut snapshot = ContextSnapshot {
            id: format!("snapshot_{}", chrono::Utc::now().timestamp()),
            timestamp: Some(chrono::Utc::now()),
            scopes: HashMap::new(),
            knowledge: HashMap::new(),
            todos: HashMap::new(),
            decisions: HashMap::new(),
            patterns: HashMap::new(),
            conventions: HashMap::new(),
            files: HashMap::new(),
            context_files: HashMap::new(),
        };

        // Walk through repository to find context files
        for entry in walkdir::WalkDir::new(repo_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && self.is_context_file(path) {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let relative_path = path
                        .strip_prefix(repo_path)
                        .unwrap_or(path)
                        .to_string_lossy()
                        .to_string();

                    match path.file_name().and_then(|s| s.to_str()) {
                        Some("rhema.yaml") => {
                            snapshot.scopes.insert(relative_path, content);
                        }
                        Some("knowledge.yaml") => {
                            snapshot.knowledge.insert(relative_path, content);
                        }
                        Some("todos.yaml") => {
                            snapshot.todos.insert(relative_path, content);
                        }
                        Some("decisions.yaml") => {
                            snapshot.decisions.insert(relative_path, content);
                        }
                        Some("patterns.yaml") => {
                            snapshot.patterns.insert(relative_path, content);
                        }
                        Some("conventions.yaml") => {
                            snapshot.conventions.insert(relative_path, content);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(snapshot)
    }

    /// Check if a file is a context file
    fn is_context_file(&self, path: &Path) -> bool {
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            matches!(
                file_name,
                "rhema.yaml"
                    | "knowledge.yaml"
                    | "todos.yaml"
                    | "decisions.yaml"
                    | "patterns.yaml"
                    | "conventions.yaml"
            )
        } else {
            false
        }
    }

    /// Create version tag
    fn create_version_tag(&self, version: &str, commit: &Commit) -> RhemaResult<()> {
        let tag_name = format!("v{}", version);
        let signature = self.repo.signature()?;

        let commit_obj = commit.as_object();
        self.repo.tag(
            &tag_name,
            commit_obj,
            &signature,
            &format!("Context version {}", version),
            false,
        )?;

        Ok(())
    }

    /// Get context version
    pub fn get_context_version(&self, version: &str) -> RhemaResult<Option<&ContextVersion>> {
        Ok(self.version_cache.get(version))
    }

    /// List all context versions
    pub fn list_context_versions(&self) -> Vec<&ContextVersion> {
        self.version_cache.values().collect()
    }

    /// Rollback to context version
    pub fn rollback_to_version(&self, version: &str) -> RhemaResult<()> {
        if let Some(context_version) = self.version_cache.get(version) {
            // Create backup of current state before rollback
            self.create_backup(&format!("before_rollback_to_{}", version))?;

            // Get the snapshot from the context version
            let snapshot = &context_version.snapshot;

            // Validate the snapshot
            if !self.validate_snapshot(snapshot)? {
                return Err(RhemaError::ValidationError(format!(
                    "Snapshot validation failed for version {}",
                    version
                )));
            }

            // Restore files from snapshot
            for (file_path, content) in &snapshot.files {
                let full_path = self
                    .repo
                    .path()
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join(file_path);

                // Create directory if it doesn't exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Write file content
                std::fs::write(&full_path, content)?;
            }

            // Restore context files
            for (file_path, content) in &snapshot.context_files {
                let full_path = self
                    .repo
                    .path()
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join(file_path);

                // Create directory if it doesn't exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Write file content
                std::fs::write(&full_path, content)?;
            }

            // Restore todos
            for (file_path, content) in &snapshot.todos {
                let full_path = self
                    .repo
                    .path()
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join(file_path);

                // Create directory if it doesn't exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Write file content
                std::fs::write(&full_path, content)?;
            }

            // Restore knowledge entries
            for (file_path, content) in &snapshot.knowledge {
                let full_path = self
                    .repo
                    .path()
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join(file_path);

                // Create directory if it doesn't exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Write file content
                std::fs::write(&full_path, content)?;
            }

            // Restore decisions
            for (file_path, content) in &snapshot.decisions {
                let full_path = self
                    .repo
                    .path()
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join(file_path);

                // Create directory if it doesn't exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Write file content
                std::fs::write(&full_path, content)?;
            }

            // Restore patterns
            for (file_path, content) in &snapshot.patterns {
                let full_path = self
                    .repo
                    .path()
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join(file_path);

                // Create directory if it doesn't exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Write file content
                std::fs::write(&full_path, content)?;
            }

            // Restore conventions
            for (file_path, content) in &snapshot.conventions {
                let full_path = self
                    .repo
                    .path()
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join(file_path);

                // Create directory if it doesn't exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Write file content
                std::fs::write(&full_path, content)?;
            }

            // Restore scopes
            for (file_path, content) in &snapshot.scopes {
                let full_path = self
                    .repo
                    .path()
                    .parent()
                    .unwrap_or_else(|| std::path::Path::new("."))
                    .join(file_path);

                // Create directory if it doesn't exist
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }

                // Write file content
                std::fs::write(&full_path, content)?;
            }

            println!("Successfully rolled back to version: {}", version);
        } else {
            return Err(RhemaError::ValidationError(format!(
                "Version {} not found in cache",
                version
            )));
        }

        Ok(())
    }

    /// Get context change history
    pub fn get_change_history(
        &self,
        file_path: &Path,
        limit: Option<usize>,
    ) -> RhemaResult<Vec<ContextEvolution>> {
        let mut history = Vec::new();
        let mut revwalk = self.repo.revwalk()?;

        revwalk.push_head()?;

        let limit = limit.unwrap_or(50);
        let mut count = 0;

        for oid in revwalk {
            if count >= limit {
                break;
            }

            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;

            if self.commit_affects_file(&commit, file_path)? {
                let evolution_entry = self.create_file_evolution_entry(&commit, file_path)?;
                history.push(evolution_entry);
                count += 1;
            }
        }

        Ok(history)
    }

    /// Check if a commit affects a specific file
    fn commit_affects_file(&self, commit: &Commit, file_path: &Path) -> RhemaResult<bool> {
        let tree = commit.tree()?;
        let parent_tree = if let Ok(parent) = commit.parent(0) {
            parent.tree().ok()
        } else {
            None
        };

        let mut diff_options = DiffOptions::new();
        let diff = if let Some(parent_tree) = parent_tree {
            self.repo
                .diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_options))?
        } else {
            self.repo
                .diff_tree_to_tree(None, Some(&tree), Some(&mut diff_options))?
        };

        for delta in diff.deltas() {
            if let Some(new_file) = delta.new_file().path() {
                if new_file == file_path {
                    return Ok(true);
                }
            }
            if let Some(old_file) = delta.old_file().path() {
                if old_file == file_path {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Create file evolution entry
    fn create_file_evolution_entry(
        &self,
        commit: &Commit,
        file_path: &Path,
    ) -> RhemaResult<ContextEvolution> {
        let file_analysis = self.analyze_file_changes(file_path)?;

        // Convert FileChangeAnalysis to Vec<ContextChange>
        let changes = vec![ContextChange {
            file_path: file_analysis.file_path,
            change_type: file_analysis.change_type,
            description: format!("File change analysis for {:?}", file_path),
            lines_added: None,
            lines_removed: None,
            specific_changes: Vec::new(),
            impact_level: file_analysis.impact_level,
            related_scopes: file_analysis.affected_scopes,
        }];

        Ok(ContextEvolution {
            commit_hash: commit.id().to_string(),
            commit_message: commit.message().unwrap_or("").to_string(),
            author: AuthorInfo {
                name: commit.author().name().unwrap_or("").to_string(),
                email: commit.author().email().unwrap_or("").to_string(),
                timestamp: DateTime::from_timestamp(commit.author().when().seconds(), 0)
                    .unwrap_or_else(|| Utc::now()),
            },
            timestamp: DateTime::from_timestamp(commit.time().seconds(), 0)
                .unwrap_or_else(|| Utc::now()),
            changes,
            impact: None,
            related_knowledge: Vec::new(),
            tags: Vec::new(),
            advanced_features: AdvancedEvolutionFeatures {
                evolution_type: EvolutionType::Custom("Unknown".to_string()),
                evolution_category: EvolutionCategory::Custom("Unknown".to_string()),
                evolution_priority: EvolutionPriority::Optional,
                evolution_complexity: EvolutionComplexity::Simple,
                evolution_risk: EvolutionRisk::None,
                evolution_dependencies: Vec::new(),
                evolution_blockers: Vec::new(),
                evolution_approvals: Vec::new(),
            },
            analytics: ContextAnalytics {
                change_frequency: 0.0,
                change_velocity: 0.0,
                impact_score: 0.0,
                complexity_score: 0.0,
                risk_score: 0.0,
                quality_score: 0.0,
                maturity_score: 0.0,
                stability_score: 0.0,
            },
            metadata: ContextMetadata {
                version: "0.0.0".to_string(),
                schema_version: "1.0.0".to_string(),
                checksum: "".to_string(),
                size: 0,
                compression_ratio: None,
                encrypted: false,
                backed_up: false,
                validation_status: ValidationStatus::Unknown,
                dependencies: Vec::new(),
                references: Vec::new(),
            },
        })
    }

    /// Analyze file changes
    fn analyze_file_changes_with_commit(
        &self,
        commit: &Commit,
        file_path: &Path,
    ) -> RhemaResult<FileChangeAnalysis> {
        let repo = &self.repo;

        // Get the commit's parent to compare against
        let parent = commit.parent(0).ok();

        // Determine change type by checking if file exists in parent and current commit
        let change_type = self.determine_change_type(repo, commit, parent.as_ref(), file_path)?;

        // Analyze the diff to get detailed information
        let (lines_added, lines_removed, specific_changes) =
            self.analyze_file_diff(repo, commit, parent.as_ref(), file_path)?;

        // Determine impact level based on various factors
        let impact_level = self.calculate_impact_level(
            &change_type,
            lines_added,
            lines_removed,
            &specific_changes,
            file_path,
        );

        // Analyze affected scopes based on file path and content
        let affected_scopes = self.analyze_affected_scopes(file_path, &specific_changes);

        // Analyze complexity changes
        let complexity_changes = self.analyze_complexity_changes(
            &change_type,
            lines_added,
            lines_removed,
            &specific_changes,
        );

        // Analyze security implications
        let security_implications =
            self.analyze_security_implications(file_path, &change_type, &specific_changes);

        // Analyze performance implications
        let performance_implications =
            self.analyze_performance_implications(file_path, &change_type, &specific_changes);

        // Generate recommendations
        let mut analysis = FileChangeAnalysis {
            file_path: file_path.to_path_buf(),
            change_type: change_type.clone(),
            impact_level: impact_level.clone(),
            affected_scopes: affected_scopes.clone(),
            complexity_changes: complexity_changes.clone(),
            security_implications: security_implications.clone(),
            performance_implications: performance_implications.clone(),
            recommendations: Vec::new(),
        };
        self.generate_file_recommendations(&mut analysis);
        let recommendations = analysis.recommendations;

        Ok(FileChangeAnalysis {
            file_path: file_path.to_path_buf(),
            change_type,
            impact_level,
            affected_scopes,
            complexity_changes,
            security_implications,
            performance_implications,
            recommendations,
        })
    }

    /// Generate context evolution report
    pub fn generate_evolution_report(
        &mut self,
        scope_path: &str,
        since: Option<DateTime<Utc>>,
    ) -> RhemaResult<EvolutionReport> {
        let evolution = self.track_context_evolution(scope_path, None)?;

        let filtered_evolution = if let Some(since) = since {
            evolution
                .into_iter()
                .filter(|e| e.timestamp >= since)
                .collect()
        } else {
            evolution
        };

        let report = EvolutionReport {
            scope_path: scope_path.to_string(),
            period: since,
            start_date: filtered_evolution
                .first()
                .map_or(Utc::now(), |e| e.timestamp),
            end_date: filtered_evolution
                .last()
                .map_or(Utc::now(), |e| e.timestamp),
            total_commits: filtered_evolution.len(),
            changes_by_type: self.analyze_changes_by_type(&filtered_evolution),
            top_contributors: self.analyze_top_contributors(&filtered_evolution),
            impact_summary: self.generate_impact_summary(&filtered_evolution),
            recommendations: self.generate_recommendations(&filtered_evolution),
        };

        Ok(report)
    }

    /// Analyze changes by type
    fn analyze_changes_by_type(
        &self,
        evolution: &[ContextEvolution],
    ) -> HashMap<ChangeType, usize> {
        let mut changes_by_type = HashMap::new();

        for entry in evolution {
            for change in &entry.changes {
                *changes_by_type
                    .entry(change.change_type.clone())
                    .or_insert(0) += 1;
            }
        }

        changes_by_type
    }

    /// Analyze top contributors
    fn analyze_top_contributors(&self, evolution: &[ContextEvolution]) -> Vec<ContributorStats> {
        let mut contributor_stats = HashMap::new();

        for entry in evolution {
            let author = &entry.author.name;
            let stats = contributor_stats
                .entry(author.clone())
                .or_insert(ContributorStats {
                    name: author.clone(),
                    commits: 0,
                    changes: 0,
                    impact_score: 0.0,
                });

            stats.commits += 1;
            stats.changes += entry.changes.len();
            stats.impact_score += self.calculate_impact_score(&entry.changes);
        }

        let mut stats: Vec<ContributorStats> = contributor_stats.into_values().collect();
        stats.sort_by(|a, b| {
            b.impact_score
                .partial_cmp(&a.impact_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        stats.truncate(10); // Top 10 contributors
        stats
    }

    /// Calculate impact score
    fn calculate_impact_score(&self, changes: &[ContextChange]) -> f64 {
        changes
            .iter()
            .map(|change| match change.impact_level {
                ImpactLevel::Low => 1.0,
                ImpactLevel::Medium => 2.0,
                ImpactLevel::High => 3.0,
                ImpactLevel::Critical => 5.0,
            })
            .sum()
    }

    /// Generate impact summary
    fn generate_impact_summary(&self, evolution: &[ContextEvolution]) -> ImpactSummary {
        let mut total_impact = 0.0;
        let mut critical_changes = 0;
        let mut affected_scopes = std::collections::HashSet::new();

        for entry in evolution {
            for change in &entry.changes {
                total_impact += match change.impact_level {
                    ImpactLevel::Low => 1.0,
                    ImpactLevel::Medium => 2.0,
                    ImpactLevel::High => 3.0,
                    ImpactLevel::Critical => 5.0,
                };

                if matches!(change.impact_level, ImpactLevel::Critical) {
                    critical_changes += 1;
                }

                for scope in &change.related_scopes {
                    affected_scopes.insert(scope.clone());
                }
            }
        }

        ImpactSummary {
            total_impact,
            critical_changes,
            affected_scopes_count: affected_scopes.len(),
            average_impact_per_commit: if evolution.is_empty() {
                0.0
            } else {
                total_impact / evolution.len() as f64
            },
        }
    }

    /// Generate recommendations
    fn generate_recommendations(&self, evolution: &[ContextEvolution]) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Analyze patterns and generate recommendations
        let critical_changes = evolution
            .iter()
            .flat_map(|e| &e.changes)
            .filter(|c| matches!(c.impact_level, ImpactLevel::Critical))
            .count();

        if critical_changes > 5 {
            recommendations.push(
                "Consider implementing stricter review processes for critical changes".to_string(),
            );
        }

        let frequent_changes = evolution.len();
        if frequent_changes > 100 {
            recommendations.push("Consider consolidating frequent small changes into larger, more meaningful commits".to_string());
        }

        recommendations
    }

    /// Determine the type of change made to a file
    fn determine_change_type(
        &self,
        repo: &Repository,
        commit: &Commit,
        parent: Option<&Commit>,
        file_path: &Path,
    ) -> RhemaResult<ChangeType> {
        let file_path_str = file_path.to_string_lossy();

        // Check if file exists in current commit
        let exists_in_commit = commit
            .tree()
            .and_then(|tree| tree.get_path(file_path))
            .is_ok();

        // Check if file exists in parent commit
        let exists_in_parent = if let Some(parent) = parent {
            parent
                .tree()
                .and_then(|tree| tree.get_path(file_path))
                .is_ok()
        } else {
            false
        };

        match (exists_in_commit, exists_in_parent) {
            (true, false) => Ok(ChangeType::Added),
            (false, true) => Ok(ChangeType::Deleted),
            (true, true) => {
                // Check if it's a rename by looking at diff
                if let Some(parent) = parent {
                    let diff = repo.diff_tree_to_tree(
                        Some(&parent.tree()?),
                        Some(&commit.tree()?),
                        Some(&mut DiffOptions::new()),
                    )?;

                    for delta in diff.deltas() {
                        if let Some(old_file) = delta.old_file().path() {
                            if let Some(new_file) = delta.new_file().path() {
                                if old_file != new_file
                                    && (old_file == file_path || new_file == file_path)
                                {
                                    return Ok(ChangeType::Renamed);
                                }
                            }
                        }
                    }
                }
                Ok(ChangeType::Modified)
            }
            (false, false) => Err(RhemaError::GitError(git2::Error::from_str(
                "File not found in either commit",
            ))),
        }
    }

    /// Analyze the diff between two commits for a specific file
    fn analyze_file_diff(
        &self,
        repo: &Repository,
        commit: &Commit,
        parent: Option<&Commit>,
        file_path: &Path,
    ) -> RhemaResult<(Option<usize>, Option<usize>, Vec<SpecificChange>)> {
        let mut lines_added = 0;
        let mut lines_removed = 0;
        let mut specific_changes = Vec::new();

        if let Some(parent) = parent {
            let diff = repo.diff_tree_to_tree(
                Some(&parent.tree()?),
                Some(&commit.tree()?),
                Some(&mut DiffOptions::new().pathspec(file_path)),
            )?;

            for delta in diff.deltas() {
                if let Some(new_file) = delta.new_file().path() {
                    if new_file == file_path {
                        lines_added = delta.new_file().size() / 80; // Rough estimate
                        lines_removed = delta.old_file().size() / 80; // Rough estimate
                        break;
                    }
                }
            }

            // Analyze specific changes in the diff
            diff.foreach(
                &mut |delta, _progress| {
                    if let Some(new_file) = delta.new_file().path() {
                        if new_file == file_path {
                            // Add a generic change entry
                            specific_changes.push(SpecificChange {
                                field: "File content".to_string(),
                                old_value: None,
                                new_value: Some(format!("Modified file: {}", file_path.display())),
                                description: "File content modified".to_string(),
                            });
                        }
                    }
                    true
                },
                None,
                None,
                None,
            )
            .ok();
        }

        Ok((
            if lines_added > 0 {
                Some(lines_added as usize)
            } else {
                None
            },
            if lines_removed > 0 {
                Some(lines_removed as usize)
            } else {
                None
            },
            specific_changes,
        ))
    }

    /// Calculate the impact level of a file change
    fn calculate_impact_level(
        &self,
        change_type: &ChangeType,
        lines_added: Option<usize>,
        lines_removed: Option<usize>,
        specific_changes: &[SpecificChange],
        file_path: &Path,
    ) -> ImpactLevel {
        let file_extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let total_changes = lines_added.unwrap_or(0) + lines_removed.unwrap_or(0);

        // Critical files that always have high impact
        let critical_files = [
            "Cargo.toml",
            "Cargo.lock",
            "package.json",
            "package-lock.json",
            "Dockerfile",
            "docker-compose.yml",
            "Makefile",
            "README.md",
            "LICENSE",
            ".gitignore",
            ".env",
            "config.json",
            "config.yaml",
        ];

        let is_critical_file = critical_files
            .iter()
            .any(|&file| file_path.file_name().and_then(|name| name.to_str()) == Some(file));

        // High impact file types
        let high_impact_extensions = ["rs", "py", "js", "ts", "java", "cpp", "c", "h", "hpp"];
        let is_high_impact_file = high_impact_extensions.contains(&file_extension);

        // Security-sensitive files
        let security_files = ["key", "pem", "crt", "p12", "keystore", "secret"];
        let is_security_file = security_files
            .iter()
            .any(|&ext| file_path.to_string_lossy().contains(ext));

        match change_type {
            ChangeType::Deleted => {
                if is_critical_file || is_security_file {
                    ImpactLevel::Critical
                } else if is_high_impact_file {
                    ImpactLevel::High
                } else {
                    ImpactLevel::Medium
                }
            }
            ChangeType::Added => {
                if is_security_file {
                    ImpactLevel::Critical
                } else if is_critical_file || is_high_impact_file {
                    ImpactLevel::High
                } else if total_changes > 100 {
                    ImpactLevel::High
                } else {
                    ImpactLevel::Medium
                }
            }
            ChangeType::Modified => {
                if is_security_file {
                    ImpactLevel::Critical
                } else if total_changes > 200 {
                    ImpactLevel::Critical
                } else if total_changes > 100 || is_critical_file {
                    ImpactLevel::High
                } else if total_changes > 50 || is_high_impact_file {
                    ImpactLevel::Medium
                } else {
                    ImpactLevel::Low
                }
            }
            ChangeType::Renamed => ImpactLevel::Medium,
            ChangeType::Moved => ImpactLevel::Medium,
        }
    }

    /// Analyze which scopes are affected by the file change
    fn analyze_affected_scopes(
        &self,
        file_path: &Path,
        specific_changes: &[SpecificChange],
    ) -> Vec<String> {
        let mut affected_scopes = Vec::new();

        // Extract scope from file path
        let components: Vec<_> = file_path.components().collect();
        let components = components.as_slice();
        if components.len() >= 2 {
            if let Some(scope_component) = components.get(1) {
                if let std::path::Component::Normal(name) = scope_component {
                    if let Some(name_str) = name.to_str() {
                        if name_str.starts_with("rhema-") || name_str.starts_with("syneidesis") {
                            affected_scopes.push(name_str.to_string());
                        }
                    }
                }
            }
        }

        // Analyze specific changes for scope references
        for change in specific_changes {
            if let Some(new_value) = &change.new_value {
                // Look for scope references in the content
                if new_value.contains("rhema-") || new_value.contains("syneidesis") {
                    // Extract scope names using regex
                    let scope_pattern =
                        regex::Regex::new(r"rhema-[a-zA-Z0-9_-]+|syneidesis[a-zA-Z0-9_-]*")
                            .unwrap();
                    for cap in scope_pattern.find_iter(new_value) {
                        affected_scopes.push(cap.as_str().to_string());
                    }
                }
            }
        }

        // Remove duplicates and sort
        affected_scopes.sort();
        affected_scopes.dedup();
        affected_scopes
    }

    /// Analyze complexity changes in the file
    fn analyze_complexity_changes(
        &self,
        change_type: &ChangeType,
        lines_added: Option<usize>,
        lines_removed: Option<usize>,
        specific_changes: &[SpecificChange],
    ) -> Vec<String> {
        let mut complexity_changes = Vec::new();

        let total_added = lines_added.unwrap_or(0);
        let total_removed = lines_removed.unwrap_or(0);
        let net_change = total_added as i32 - total_removed as i32;

        match change_type {
            ChangeType::Added => {
                if total_added > 100 {
                    complexity_changes.push(
                        "Large new file added - consider breaking into smaller modules".to_string(),
                    );
                }
                if total_added > 50 {
                    complexity_changes.push("Significant new functionality added".to_string());
                }
            }
            ChangeType::Deleted => {
                complexity_changes
                    .push("File removed - ensure no dependencies are broken".to_string());
            }
            ChangeType::Modified => {
                if net_change > 50 {
                    complexity_changes.push("Significant increase in code complexity".to_string());
                } else if net_change < -50 {
                    complexity_changes.push("Significant reduction in code complexity".to_string());
                }

                if total_added > 100 {
                    complexity_changes
                        .push("Large number of lines added - consider code review".to_string());
                }

                if total_removed > 100 {
                    complexity_changes.push("Large refactoring detected".to_string());
                }
            }
            ChangeType::Renamed => {
                complexity_changes.push("File renamed - update all references".to_string());
            }
            ChangeType::Moved => {
                complexity_changes
                    .push("File moved - update import paths and dependencies".to_string());
            }
        }

        // Analyze specific changes for complexity indicators
        let function_pattern = regex::Regex::new(r"fn\s+\w+").unwrap();
        let struct_pattern = regex::Regex::new(r"struct\s+\w+").unwrap();
        let trait_pattern = regex::Regex::new(r"trait\s+\w+").unwrap();

        let mut functions_added = 0;
        let mut structs_added = 0;
        let mut traits_added = 0;

        for change in specific_changes {
            if let Some(new_value) = &change.new_value {
                if function_pattern.is_match(new_value) {
                    functions_added += 1;
                }
                if struct_pattern.is_match(new_value) {
                    structs_added += 1;
                }
                if trait_pattern.is_match(new_value) {
                    traits_added += 1;
                }
            }
        }

        if functions_added > 5 {
            complexity_changes.push(format!(
                "{} new functions added - consider module organization",
                functions_added
            ));
        }
        if structs_added > 2 {
            complexity_changes.push(format!(
                "{} new structs added - review data model design",
                structs_added
            ));
        }
        if traits_added > 1 {
            complexity_changes.push(format!(
                "{} new traits added - review interface design",
                traits_added
            ));
        }

        complexity_changes
    }

    /// Analyze security implications of the file change
    fn analyze_security_implications(
        &self,
        file_path: &Path,
        change_type: &ChangeType,
        specific_changes: &[SpecificChange],
    ) -> Vec<String> {
        let mut security_implications = Vec::new();

        let file_name = file_path.to_string_lossy().to_lowercase();

        // Check for security-sensitive file types
        let security_extensions = ["key", "pem", "crt", "p12", "keystore", "secret", "token"];
        let is_security_file = security_extensions
            .iter()
            .any(|&ext| file_name.contains(ext));

        if is_security_file {
            match change_type {
                ChangeType::Added => {
                    security_implications.push(
                        "Security credentials added - ensure proper access controls".to_string(),
                    );
                }
                ChangeType::Modified => {
                    security_implications.push(
                        "Security credentials modified - review access permissions".to_string(),
                    );
                }
                ChangeType::Deleted => {
                    security_implications.push(
                        "Security credentials removed - verify no active dependencies".to_string(),
                    );
                }
                _ => {}
            }
        }

        // Check for security-sensitive patterns in code changes
        let security_patterns = [
            ("password", "Password handling detected"),
            ("secret", "Secret handling detected"),
            ("token", "Token handling detected"),
            ("key", "Key handling detected"),
            ("auth", "Authentication code modified"),
            ("crypto", "Cryptographic code modified"),
            ("hash", "Hash function usage detected"),
            ("encrypt", "Encryption code detected"),
            ("decrypt", "Decryption code detected"),
        ];

        for change in specific_changes {
            if let Some(new_value) = &change.new_value {
                let new_value_lower = new_value.to_lowercase();
                for (pattern, message) in security_patterns {
                    if new_value_lower.contains(pattern) {
                        security_implications
                            .push(format!("{} - review security implications", message));
                    }
                }
            }
        }

        // Check for potential security vulnerabilities
        let vulnerability_patterns = [
            ("unsafe", "Unsafe code block detected"),
            ("raw pointer", "Raw pointer usage detected"),
            ("unchecked", "Unchecked operation detected"),
            ("panic!", "Panic macro usage detected"),
            (
                "unwrap()",
                "Unwrap usage detected - consider proper error handling",
            ),
        ];

        for change in specific_changes {
            if let Some(new_value) = &change.new_value {
                for (pattern, message) in vulnerability_patterns {
                    if new_value.contains(pattern) {
                        security_implications.push(message.to_string());
                    }
                }
            }
        }

        security_implications
    }

    /// Analyze performance implications of the file change
    fn analyze_performance_implications(
        &self,
        file_path: &Path,
        change_type: &ChangeType,
        specific_changes: &[SpecificChange],
    ) -> Vec<String> {
        let mut performance_implications = Vec::new();

        let file_extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        // Performance-sensitive file types
        let performance_extensions = ["rs", "py", "js", "ts", "java", "cpp", "c"];
        let is_performance_file = performance_extensions.contains(&file_extension);

        if is_performance_file {
            // Check for performance-related patterns
            let performance_patterns = [
                ("loop", "Loop construct detected - review complexity"),
                ("for", "For loop detected - consider iterator optimization"),
                (
                    "while",
                    "While loop detected - ensure termination condition",
                ),
                ("recursion", "Recursion detected - check for stack overflow"),
                ("async", "Async code detected - review concurrency patterns"),
                (
                    "await",
                    "Await usage detected - check for blocking operations",
                ),
                (
                    "clone",
                    "Clone operation detected - consider reference usage",
                ),
                ("copy", "Copy operation detected - review memory usage"),
                ("Box", "Box allocation detected - consider stack allocation"),
                ("Arc", "Arc usage detected - review thread safety"),
                ("Mutex", "Mutex usage detected - check for deadlocks"),
                ("RwLock", "RwLock usage detected - review locking strategy"),
            ];

            for change in specific_changes {
                if let Some(new_value) = &change.new_value {
                    for (pattern, message) in performance_patterns {
                        if new_value.contains(pattern) {
                            performance_implications.push(message.to_string());
                        }
                    }
                }
            }

            // Check for potential performance issues
            let issue_patterns = [
                ("O(n²)", "Quadratic complexity detected"),
                ("O(n³)", "Cubic complexity detected"),
                ("exponential", "Exponential complexity detected"),
                ("infinite loop", "Potential infinite loop detected"),
                ("memory leak", "Potential memory leak detected"),
            ];

            for change in specific_changes {
                if let Some(new_value) = &change.new_value {
                    for (pattern, message) in issue_patterns {
                        if new_value.to_lowercase().contains(pattern) {
                            performance_implications.push(message.to_string());
                        }
                    }
                }
            }
        }

        // Database-related performance implications
        let file_name = file_path.to_string_lossy().to_lowercase();
        if file_name.contains("database") || file_name.contains("db") || file_name.contains("sql") {
            performance_implications
                .push("Database-related changes detected - review query performance".to_string());
        }

        // Network-related performance implications
        if file_name.contains("network") || file_name.contains("http") || file_name.contains("api")
        {
            performance_implications
                .push("Network-related changes detected - review latency implications".to_string());
        }

        performance_implications
    }
}

/// Evolution report
#[derive(Debug, Clone)]
pub struct EvolutionReport {
    pub scope_path: String,
    pub period: Option<DateTime<Utc>>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_commits: usize,
    pub changes_by_type: HashMap<ChangeType, usize>,
    pub top_contributors: Vec<ContributorStats>,
    pub impact_summary: ImpactSummary,
    pub recommendations: Vec<String>,
}

/// Contributor statistics
#[derive(Debug, Clone)]
pub struct ContributorStats {
    pub name: String,
    pub commits: usize,
    pub changes: usize,
    pub impact_score: f64,
}

/// Impact summary
#[derive(Debug, Clone)]
pub struct ImpactSummary {
    pub total_impact: f64,
    pub critical_changes: usize,
    pub affected_scopes_count: usize,
    pub average_impact_per_commit: f64,
}

/// File change analysis
#[derive(Debug, Clone)]
pub struct FileChangeAnalysis {
    pub file_path: PathBuf,
    pub change_type: ChangeType,
    pub impact_level: ImpactLevel,
    pub affected_scopes: Vec<String>,
    pub complexity_changes: Vec<String>,
    pub security_implications: Vec<String>,
    pub performance_implications: Vec<String>,
    pub recommendations: Vec<String>,
}
