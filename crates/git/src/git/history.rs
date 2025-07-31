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

use rhema_core::{RhemaError, RhemaResult};
use git2::{Repository, Commit, DiffOptions, BlameOptions};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn track_context_evolution(&mut self, scope_path: &str, limit: Option<usize>) -> RhemaResult<Vec<ContextEvolution>> {
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
            self.repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_options))?
        } else {
            self.repo.diff_tree_to_tree(None, Some(&tree), Some(&mut diff_options))?
        };
        
        for delta in diff.deltas() {
            if let Some(new_file) = delta.new_file().path() {
                if new_file.to_string_lossy().contains(scope_path) {
                    return Ok(true);
                }
            }
            if let Some(old_file) = delta.old_file().path() {
                if old_file.to_string_lossy().contains(scope_path) {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Create evolution entry from commit
    fn create_evolution_entry(&self, commit: &Commit, scope_path: &str) -> RhemaResult<ContextEvolution> {
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
    fn analyze_commit_changes(&self, commit: &Commit, scope_path: &str) -> RhemaResult<Vec<ContextChange>> {
        let mut changes = Vec::new();
        let tree = commit.tree()?;
        let parent_tree = if let Ok(parent) = commit.parent(0) {
            parent.tree().ok()
        } else {
            None
        };
        
        let mut diff_options = DiffOptions::new();
        let diff = if let Some(parent_tree) = parent_tree {
            self.repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_options))?
        } else {
            self.repo.diff_tree_to_tree(None, Some(&tree), Some(&mut diff_options))?
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
    fn create_context_change(&self, delta: git2::DiffDelta, file_path: &Path) -> RhemaResult<ContextChange> {
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
    fn calculate_line_changes(&self, _delta: &git2::DiffDelta) -> RhemaResult<(Option<usize>, Option<usize>)> {
        // TODO: Implement line change calculation
        // This would analyze the actual diff to count added/removed lines
        Ok((None, None))
    }
    
    /// Analyze specific changes
    fn analyze_specific_changes(&self, _delta: &git2::DiffDelta) -> RhemaResult<Vec<SpecificChange>> {
        // TODO: Implement specific change analysis
        // This would parse the diff to identify specific field changes
        Ok(Vec::new())
    }
    
    /// Assess impact level
    fn assess_impact_level(&self, _delta: &git2::DiffDelta) -> ImpactLevel {
        // TODO: Implement impact level assessment
        // This would analyze the type and scope of changes
        ImpactLevel::Medium
    }
    
    /// Extract related scopes
    fn extract_related_scopes(&self, _file_path: &Path) -> Vec<String> {
        // TODO: Implement scope extraction
        // This would analyze the file path and content to identify related scopes
        Vec::new()
    }
    
    /// Analyze commit impact
    fn analyze_commit_impact(&self, _commit: &Commit, _scope_path: &str) -> RhemaResult<Option<ImpactAnalysis>> {
        // TODO: Implement impact analysis
        // This would analyze the broader impact of the commit
        Ok(None)
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
            let line_content = self.get_line_content(file_path, hunk.final_start_line().try_into().unwrap_or(0))?;
            
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
                entry_type: self.extract_entry_type(&line_content),
                entry_id: self.extract_entry_id(&line_content),
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
        
        self.blame_cache.insert(file_path.to_path_buf(), context_blame.clone());
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
    
    /// Extract entry type from line content
    fn extract_entry_type(&self, _content: &str) -> Option<String> {
        // TODO: Implement entry type extraction
        // This would parse the YAML content to identify entry types
        None
    }
    
    /// Extract entry ID from line content
    fn extract_entry_id(&self, _content: &str) -> Option<String> {
        // TODO: Implement entry ID extraction
        // This would parse the YAML content to identify entry IDs
        None
    }
    
    /// Create context version
    pub fn create_context_version(&mut self, version: &str, version_type: VersionType, description: &str) -> RhemaResult<ContextVersion> {
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
        
        self.version_cache.insert(version.to_string(), context_version.clone());
        
        // Create Git tag for the version
        self.create_version_tag(version, &commit)?;
        
        Ok(context_version)
    }
    
    /// Create context snapshot
    fn create_context_snapshot(&self) -> RhemaResult<ContextSnapshot> {
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| RhemaError::GitError(git2::Error::from_str("Invalid repository path")))?;
        
        let mut snapshot = ContextSnapshot {
            scopes: HashMap::new(),
            knowledge: HashMap::new(),
            todos: HashMap::new(),
            decisions: HashMap::new(),
            patterns: HashMap::new(),
            conventions: HashMap::new(),
        };
        
        // Walk through repository to find context files
        for entry in walkdir::WalkDir::new(repo_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && self.is_context_file(path) {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let relative_path = path.strip_prefix(repo_path)
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
            matches!(file_name, 
                "rhema.yaml" | 
                "knowledge.yaml" | 
                "todos.yaml" | 
                "decisions.yaml" | 
                "patterns.yaml" | 
                "conventions.yaml"
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
        self.repo.tag(&tag_name, commit_obj, &signature, &format!("Context version {}", version), false)?;
        
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
        if let Some(_context_version) = self.version_cache.get(version) {
            // TODO: Implement rollback logic
            // This would restore the context files to the specified version
            println!("Rolling back to version: {}", version);
        }
        
        Ok(())
    }
    
    /// Get context change history
    pub fn get_change_history(&self, file_path: &Path, limit: Option<usize>) -> RhemaResult<Vec<ContextEvolution>> {
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
            self.repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut diff_options))?
        } else {
            self.repo.diff_tree_to_tree(None, Some(&tree), Some(&mut diff_options))?
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
    fn create_file_evolution_entry(&self, commit: &Commit, file_path: &Path) -> RhemaResult<ContextEvolution> {
        let changes = self.analyze_file_changes(commit, file_path)?;
        
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
    fn analyze_file_changes(&self, _commit: &Commit, _file_path: &Path) -> RhemaResult<Vec<ContextChange>> {
        // TODO: Implement file-specific change analysis
        Ok(Vec::new())
    }
    
    /// Generate context evolution report
    pub fn generate_evolution_report(&mut self, scope_path: &str, since: Option<DateTime<Utc>>) -> RhemaResult<EvolutionReport> {
        let evolution = self.track_context_evolution(scope_path, None)?;
        
        let filtered_evolution = if let Some(since) = since {
            evolution.into_iter()
                .filter(|e| e.timestamp >= since)
                .collect()
        } else {
            evolution
        };
        
        let report = EvolutionReport {
            scope_path: scope_path.to_string(),
            period: since,
            start_date: filtered_evolution.first().map_or(Utc::now(), |e| e.timestamp),
            end_date: filtered_evolution.last().map_or(Utc::now(), |e| e.timestamp),
            total_commits: filtered_evolution.len(),
            changes_by_type: self.analyze_changes_by_type(&filtered_evolution),
            top_contributors: self.analyze_top_contributors(&filtered_evolution),
            impact_summary: self.generate_impact_summary(&filtered_evolution),
            recommendations: self.generate_recommendations(&filtered_evolution),
        };
        
        Ok(report)
    }
    
    /// Analyze changes by type
    fn analyze_changes_by_type(&self, evolution: &[ContextEvolution]) -> HashMap<ChangeType, usize> {
        let mut changes_by_type = HashMap::new();
        
        for entry in evolution {
            for change in &entry.changes {
                *changes_by_type.entry(change.change_type.clone()).or_insert(0) += 1;
            }
        }
        
        changes_by_type
    }
    
    /// Analyze top contributors
    fn analyze_top_contributors(&self, evolution: &[ContextEvolution]) -> Vec<ContributorStats> {
        let mut contributor_stats = HashMap::new();
        
        for entry in evolution {
            let author = &entry.author.name;
            let stats = contributor_stats.entry(author.clone()).or_insert(ContributorStats {
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
        stats.sort_by(|a, b| b.impact_score.partial_cmp(&a.impact_score).unwrap_or(std::cmp::Ordering::Equal));
        
        stats.truncate(10); // Top 10 contributors
        stats
    }
    
    /// Calculate impact score
    fn calculate_impact_score(&self, changes: &[ContextChange]) -> f64 {
        changes.iter().map(|change| {
            match change.impact_level {
                ImpactLevel::Low => 1.0,
                ImpactLevel::Medium => 2.0,
                ImpactLevel::High => 3.0,
                ImpactLevel::Critical => 5.0,
            }
        }).sum()
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
            average_impact_per_commit: if evolution.is_empty() { 0.0 } else { total_impact / evolution.len() as f64 },
        }
    }
    
    /// Generate recommendations
    fn generate_recommendations(&self, evolution: &[ContextEvolution]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // Analyze patterns and generate recommendations
        let critical_changes = evolution.iter()
            .flat_map(|e| &e.changes)
            .filter(|c| matches!(c.impact_level, ImpactLevel::Critical))
            .count();
        
        if critical_changes > 5 {
            recommendations.push("Consider implementing stricter review processes for critical changes".to_string());
        }
        
        let frequent_changes = evolution.len();
        if frequent_changes > 100 {
            recommendations.push("Consider consolidating frequent small changes into larger, more meaningful commits".to_string());
        }
        
        recommendations
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
