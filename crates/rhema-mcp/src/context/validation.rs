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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Validation error types for context data
#[derive(Debug, Clone, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum ValidationErrorType {
    SchemaViolation,
    CrossReferenceError,
    ConsistencyError,
    TemporalError,
    DependencyError,
    SecurityError,
    DataIntegrityError,
    DuplicateEntry,
    InvalidReference,
    MissingRequiredField,
}

/// Validation severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Individual validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub scope_path: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub message: String,
    pub severity: ValidationSeverity,
    pub field_path: Option<String>,
    pub suggested_fix: Option<String>,
}

/// Validation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub warning_type: String,
    pub scope_path: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub message: String,
    pub field_path: Option<String>,
    pub recommendation: Option<String>,
}

/// Validation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStats {
    pub total_entries_validated: usize,
    pub errors_count: usize,
    pub warnings_count: usize,
    pub validation_time_ms: u64,
    pub memory_usage_bytes: u64,
    pub validation_score: f64, // 0.0 to 1.0
}

/// Comprehensive validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextValidationResult {
    pub is_valid: bool,
    pub validation_errors: Vec<ValidationError>,
    pub validation_warnings: Vec<ValidationWarning>,
    pub validation_stats: ValidationStats,
    pub recommendations: Vec<String>,
    pub validated_at: DateTime<Utc>,
    pub scope_validation_results: HashMap<String, ScopeValidationResult>,
}

/// Scope-specific validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeValidationResult {
    pub scope_path: String,
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub knowledge_valid: bool,
    pub todos_valid: bool,
    pub decisions_valid: bool,
    pub patterns_valid: bool,
    pub conventions_valid: bool,
}

/// Cross-reference validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReferenceValidation {
    pub is_valid: bool,
    pub broken_references: Vec<BrokenReference>,
    pub orphaned_entries: Vec<OrphanedEntry>,
    pub circular_references: Vec<CircularReference>,
}

/// Broken reference information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenReference {
    pub source_scope: String,
    pub source_resource: String,
    pub source_id: String,
    pub referenced_scope: String,
    pub referenced_resource: String,
    pub referenced_id: String,
    pub reference_type: String,
}

/// Orphaned entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanedEntry {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub entry_title: String,
    pub orphaned_since: DateTime<Utc>,
}

/// Circular reference information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularReference {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub circular_path: Vec<String>,
}

/// Consistency validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyValidation {
    pub is_consistent: bool,
    pub naming_conflicts: Vec<NamingConflict>,
    pub duplicate_entries: Vec<DuplicateEntry>,
    pub conflicting_information: Vec<ConflictingInformation>,
}

/// Naming conflict information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamingConflict {
    pub conflict_type: String,
    pub conflicting_names: Vec<String>,
    pub affected_scopes: Vec<String>,
    pub severity: ValidationSeverity,
}

/// Duplicate entry information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateEntry {
    pub resource_type: String,
    pub entry_id: String,
    pub duplicate_ids: Vec<String>,
    pub affected_scopes: Vec<String>,
    pub similarity_score: f64,
}

/// Conflicting information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictingInformation {
    pub field_name: String,
    pub conflicting_values: Vec<String>,
    pub affected_scopes: Vec<String>,
    pub conflict_type: String,
}

/// Temporal validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalValidation {
    pub is_temporally_consistent: bool,
    pub future_dates_in_past: Vec<TemporalAnomaly>,
    pub invalid_timestamp_sequences: Vec<TimestampSequence>,
    pub expired_entries: Vec<ExpiredEntry>,
}

/// Temporal anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalAnomaly {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub field_name: String,
    pub future_date: DateTime<Utc>,
    pub current_date: DateTime<Utc>,
}

/// Timestamp sequence issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampSequence {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub earlier_field: String,
    pub later_field: String,
    pub earlier_timestamp: DateTime<Utc>,
    pub later_timestamp: DateTime<Utc>,
}

/// Expired entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiredEntry {
    pub scope_path: String,
    pub resource_type: String,
    pub entry_id: String,
    pub expiry_field: String,
    pub expiry_date: DateTime<Utc>,
    pub days_expired: i64,
}

/// Dependency validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyValidation {
    pub is_valid: bool,
    pub circular_dependencies: Vec<String>,
    pub missing_dependencies: Vec<String>,
    pub version_conflicts: Vec<String>,
    pub unresolved_dependencies: Vec<String>,
    pub validation_time_ms: u64,
}

/// Version validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionValidation {
    pub is_valid: bool,
    pub incompatible_versions: Vec<String>,
    pub breaking_changes: Vec<String>,
    pub migration_required: Vec<String>,
    pub validation_time_ms: u64,
}
