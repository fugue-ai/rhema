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

use crate::{RhemaError, scope::Scope};
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use regex::Regex;
use chrono::{DateTime, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};

/// Provenance information for query execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryProvenance {
    /// Original query string
    pub original_query: String,
    
    /// Parsed query structure
    pub parsed_query: CqlQuery,
    
    /// Execution timestamp
    pub executed_at: DateTime<Utc>,
    
    /// Execution duration in milliseconds
    pub execution_time_ms: u64,
    
    /// Scopes that were searched
    pub scopes_searched: Vec<String>,
    
    /// Files that were accessed
    pub files_accessed: Vec<String>,
    
    /// Query execution steps
    pub execution_steps: Vec<ExecutionStep>,
    
    /// Applied filters and conditions
    pub applied_filters: Vec<AppliedFilter>,
    
    /// Performance metrics
    pub performance_metrics: PerformanceMetrics,
    
    /// Error information if any
    pub errors: Option<Vec<String>>,
}

/// Individual execution step in query processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStep {
    /// Step name/description
    pub name: String,
    
    /// Step type
    pub step_type: ExecutionStepType,
    
    /// Duration in milliseconds
    pub duration_ms: u64,
    
    /// Input data size (bytes)
    pub input_size: Option<usize>,
    
    /// Output data size (bytes)
    pub output_size: Option<usize>,
    
    /// Step-specific metadata
    pub metadata: HashMap<String, Value>,
}

/// Types of execution steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStepType {
    QueryParsing,
    ScopeResolution,
    FileAccess,
    YamlPathExtraction,
    ConditionFiltering,
    Ordering,
    LimitOffset,
    DataTransformation,
    ResultAssembly,
}

/// Applied filter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedFilter {
    /// Filter type
    pub filter_type: FilterType,
    
    /// Filter description
    pub description: String,
    
    /// Number of items before filter
    pub items_before: usize,
    
    /// Number of items after filter
    pub items_after: usize,
    
    /// Filter parameters
    pub parameters: HashMap<String, Value>,
}

/// Types of filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterType {
    WhereCondition,
    YamlPath,
    OrderBy,
    Limit,
    Offset,
    ScopeFilter,
}

impl std::fmt::Display for FilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterType::WhereCondition => write!(f, "where_condition"),
            FilterType::YamlPath => write!(f, "yaml_path"),
            FilterType::OrderBy => write!(f, "order_by"),
            FilterType::Limit => write!(f, "limit"),
            FilterType::Offset => write!(f, "offset"),
            FilterType::ScopeFilter => write!(f, "scope_filter"),
        }
    }
}

/// Performance metrics for query execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Total execution time in milliseconds
    pub total_time_ms: u64,
    
    /// Time spent in each phase
    pub phase_times: HashMap<String, u64>,
    
    /// Memory usage in bytes
    pub memory_usage_bytes: Option<usize>,
    
    /// Number of files read
    pub files_read: usize,
    
    /// Number of YAML documents processed
    pub yaml_documents_processed: usize,
    
    /// Cache hit/miss statistics
    pub cache_stats: Option<CacheStats>,
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub hit_rate: f64,
}

/// Field-level provenance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldProvenance {
    /// Field name/path
    pub field_path: String,
    
    /// Source scope
    pub source_scope: String,
    
    /// Source file
    pub source_file: String,
    
    /// Original YAML path in source file
    pub source_yaml_path: Option<String>,
    
    /// Data type of the field
    pub data_type: String,
    
    /// Whether the field was transformed
    pub was_transformed: bool,
    
    /// Transformation history
    pub transformations: Vec<Transformation>,
    
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    
    /// Last modified timestamp
    pub last_modified: Option<DateTime<Utc>>,
    
    /// Field metadata
    pub metadata: HashMap<String, Value>,
}

/// Data transformation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transformation {
    /// Transformation type
    pub transformation_type: TransformationType,
    
    /// Transformation description
    pub description: String,
    
    /// Applied at timestamp
    pub applied_at: DateTime<Utc>,
    
    /// Transformation parameters
    pub parameters: HashMap<String, Value>,
    
    /// Input value (if available)
    pub input_value: Option<Value>,
    
    /// Output value (if available)
    pub output_value: Option<Value>,
}

/// Types of transformations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationType {
    TypeConversion,
    ValueFiltering,
    Aggregation,
    Sorting,
    FieldMapping,
    DataEnrichment,
    Validation,
    Custom,
}

/// CQL query structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CqlQuery {
    /// Target file or path
    pub target: String,
    
    /// YAML path within the file
    pub yaml_path: Option<String>,
    
    /// WHERE clause conditions
    pub conditions: Vec<Condition>,
    
    /// Scope context for relative paths
    pub scope_context: Option<String>,
    
    /// ORDER BY clause
    pub order_by: Option<Vec<OrderBy>>,
    
    /// LIMIT clause
    pub limit: Option<usize>,
    
    /// OFFSET clause
    pub offset: Option<usize>,
}

/// Query condition with enhanced operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: Operator,
    pub value: ConditionValue,
    pub logical_op: LogicalOperator,
}

/// Supported comparison operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Like,
    NotLike,
    In,
    NotIn,
    Contains,
    NotContains,
    IsNull,
    IsNotNull,
}

/// Logical operators for combining conditions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

/// Condition value with type support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Array(Vec<ConditionValue>),
    DateTime(DateTime<Utc>),
}

/// ORDER BY clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBy {
    pub field: String,
    pub direction: OrderDirection,
}

/// Sort direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderDirection {
    Asc,
    Desc,
}

/// Query result with enhanced metadata and provenance
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub scope: String,
    pub file: String,
    pub data: Value,
    pub path: String,
    
    /// Field-level provenance information
    pub field_provenance: HashMap<String, FieldProvenance>,
    
    /// Query-level provenance information
    pub query_provenance: Option<QueryProvenance>,
    
    /// Result metadata
    pub metadata: HashMap<String, Value>,
}

/// Execute a CQL query
pub fn execute_query(repo_root: &Path, query: &str) -> Result<Value, RhemaError> {
    let parsed_query = parse_cql_query(query)?;
    let scopes = crate::scope::discover_scopes(repo_root)?;
    
    let results = execute_parsed_query(&parsed_query, &scopes, repo_root)?;
    
    // Convert results to a single Value
    if results.len() == 1 {
        Ok(results[0].data.clone())
    } else {
        let mut result_array = Vec::new();
        for result in results {
            let mut result_obj = HashMap::new();
            result_obj.insert("scope".to_string(), Value::String(result.scope));
            result_obj.insert("file".to_string(), Value::String(result.file));
            result_obj.insert("path".to_string(), Value::String(result.path));
            result_obj.insert("data".to_string(), result.data);
            result_array.push(Value::Mapping(serde_yaml::Mapping::from_iter(
                result_obj.into_iter().map(|(k, v)| (Value::String(k), v))
            )));
        }
        Ok(Value::Sequence(result_array))
    }
}

/// Execute a CQL query with full provenance tracking
pub fn execute_query_with_provenance(repo_root: &Path, query: &str) -> Result<(Value, QueryProvenance), RhemaError> {
    let start_time = std::time::Instant::now();
    let executed_at = Utc::now();
    
    // Parse query with timing
    let parse_start = std::time::Instant::now();
    let parsed_query = parse_cql_query(query)?;
    let parse_duration = parse_start.elapsed().as_millis() as u64;
    
    // Discover scopes with timing
    let scope_start = std::time::Instant::now();
    let scopes = crate::scope::discover_scopes(repo_root)?;
    let scope_duration = scope_start.elapsed().as_millis() as u64;
    
    // Execute query with provenance tracking
    let results = execute_parsed_query_with_provenance(&parsed_query, &scopes, repo_root, &executed_at)?;
    
    // Calculate total execution time
    let total_duration = start_time.elapsed().as_millis() as u64;
    
    // Build provenance information
    let provenance = build_query_provenance(
        query,
        &parsed_query,
        executed_at,
        total_duration,
        &scopes,
        &results,
        parse_duration,
        scope_duration,
    )?;
    
    // Convert results to a single Value (same as original)
    let result_value = if results.len() == 1 {
        results[0].data.clone()
    } else {
        let mut result_array = Vec::new();
        for result in results {
            let mut result_obj = HashMap::new();
            result_obj.insert("scope".to_string(), Value::String(result.scope));
            result_obj.insert("file".to_string(), Value::String(result.file));
            result_obj.insert("path".to_string(), Value::String(result.path));
            result_obj.insert("data".to_string(), result.data);
            result_array.push(Value::Mapping(serde_yaml::Mapping::from_iter(
                result_obj.into_iter().map(|(k, v)| (Value::String(k), v))
            )));
        }
        Value::Sequence(result_array)
    };
    
    Ok((result_value, provenance))
}

/// Parse a CQL query string with enhanced syntax
pub fn parse_cql_query(query: &str) -> Result<CqlQuery, RhemaError> {
    let query = query.trim();
    
    // Enhanced regex-based parser for CQL syntax
    let re = Regex::new(r"^([^\s]+)(?:\s+WHERE\s+(.+?))?(?:\s+ORDER\s+BY\s+(.+?))?(?:\s+LIMIT\s+(\d+))?(?:\s+OFFSET\s+(\d+))?$").map_err(|_| {
        RhemaError::InvalidQuery("Invalid regex pattern".to_string())
    })?;
    
    let captures = re.captures(query).ok_or_else(|| {
        RhemaError::InvalidQuery(format!("Invalid query syntax: {}", query))
    })?;
    
    let target = captures[1].to_string();
    let where_clause = captures.get(2).map(|m| m.as_str().to_string());
    let order_by_clause = captures.get(3).map(|m| m.as_str().to_string());
    let limit = captures.get(4).and_then(|m| m.as_str().parse::<usize>().ok());
    let offset = captures.get(5).and_then(|m| m.as_str().parse::<usize>().ok());
    
    // Parse target into file and yaml_path
    let (file, yaml_path) = parse_target(&target)?;
    
    // Parse WHERE conditions
    let conditions = if let Some(where_clause) = where_clause {
        parse_enhanced_conditions(&where_clause)?
    } else {
        Vec::new()
    };
    
    // Parse ORDER BY clause
    let order_by = if let Some(order_by_clause) = order_by_clause {
        parse_order_by(&order_by_clause)?
    } else {
        None
    };
    
    Ok(CqlQuery {
        target: file,
        yaml_path,
        conditions,
        scope_context: None,
        order_by,
        limit,
        offset,
    })
}

/// Parse target into file and YAML path
fn parse_target(target: &str) -> Result<(String, Option<String>), RhemaError> {
    if target.contains('.') {
        let parts: Vec<&str> = target.splitn(2, '.').collect();
        if parts.len() == 2 {
            Ok((parts[0].to_string(), Some(parts[1].to_string())))
        } else {
            Ok((target.to_string(), None))
        }
    } else {
        Ok((target.to_string(), None))
    }
}

/// Parse enhanced WHERE conditions with logical operators
fn parse_enhanced_conditions(where_clause: &str) -> Result<Vec<Condition>, RhemaError> {
    let mut conditions = Vec::new();
    let mut current_conditions = Vec::new();
    
    // Split by logical operators while preserving them
    let parts: Vec<&str> = where_clause.split_whitespace().collect();
    let mut i = 0;
    
    while i < parts.len() {
        let part = parts[i];
        
        match part.to_uppercase().as_str() {
            "AND" | "OR" => {
                // Process accumulated conditions
                if !current_conditions.is_empty() {
                    let condition = parse_condition_group(&current_conditions.join(" "))?;
                    conditions.push(condition);
                    current_conditions.clear();
                }
                
                // Set logical operator for next condition
                let logical_op = if part.to_uppercase() == "AND" {
                    LogicalOperator::And
                } else {
                    LogicalOperator::Or
                };
                
                // Get next condition
                i += 1;
                if i < parts.len() {
                    let mut next_condition = Vec::new();
                    while i < parts.len() && !["AND", "OR"].contains(&parts[i].to_uppercase().as_str()) {
                        next_condition.push(parts[i]);
                        i += 1;
                    }
                    if !next_condition.is_empty() {
                        let mut condition = parse_condition_group(&next_condition.join(" "))?;
                        condition.logical_op = logical_op;
                        conditions.push(condition);
                    }
                }
            }
            _ => {
                current_conditions.push(part);
                i += 1;
            }
        }
    }
    
    // Process remaining conditions
    if !current_conditions.is_empty() {
        let condition = parse_condition_group(&current_conditions.join(" "))?;
        conditions.push(condition);
    }
    
    Ok(conditions)
}

/// Parse a single condition group
fn parse_condition_group(condition: &str) -> Result<Condition, RhemaError> {
    // Handle IS NULL and IS NOT NULL
    if condition.to_uppercase().ends_with(" IS NULL") {
        let field = condition[..condition.len() - 9].trim();
        return Ok(Condition {
            field: field.to_string(),
            operator: Operator::IsNull,
            value: ConditionValue::Null,
            logical_op: LogicalOperator::And,
        });
    }
    
    if condition.to_uppercase().ends_with(" IS NOT NULL") {
        let field = condition[..condition.len() - 12].trim();
        return Ok(Condition {
            field: field.to_string(),
            operator: Operator::IsNotNull,
            value: ConditionValue::Null,
            logical_op: LogicalOperator::And,
        });
    }
    
    // Enhanced regex for various operators
    let operator_patterns = [
        (r"^(.+?)\s*!=\s*(.+)$", Operator::NotEquals),
        (r"^(.+?)\s*<=\s*(.+)$", Operator::LessThanOrEqual),
        (r"^(.+?)\s*>=\s*(.+)$", Operator::GreaterThanOrEqual),
        (r"^(.+?)\s*<\s*(.+)$", Operator::LessThan),
        (r"^(.+?)\s*>\s*(.+)$", Operator::GreaterThan),
        (r"^(.+?)\s+NOT\s+LIKE\s+(.+)$", Operator::NotLike),
        (r"^(.+?)\s+LIKE\s+(.+)$", Operator::Like),
        (r"^(.+?)\s+NOT\s+IN\s*\((.+)\)$", Operator::NotIn),
        (r"^(.+?)\s+IN\s*\((.+)\)$", Operator::In),
        (r"^(.+?)\s+NOT\s+CONTAINS\s+(.+)$", Operator::NotContains),
        (r"^(.+?)\s+CONTAINS\s+(.+)$", Operator::Contains),
        (r"^(.+?)\s*=\s*(.+)$", Operator::Equals),
    ];
    
    for (pattern, operator) in operator_patterns {
        if let Ok(re) = Regex::new(pattern) {
            if let Some(captures) = re.captures(condition) {
                let field = captures[1].trim();
                let value_str = captures[2].trim();
                
                let value = parse_condition_value(value_str, &operator)?;
                
                return Ok(Condition {
                    field: field.to_string(),
                    operator,
                    value,
                    logical_op: LogicalOperator::And,
                });
            }
        }
    }
    
    Err(RhemaError::InvalidQuery(format!("Invalid condition syntax: {}", condition)))
}

/// Parse condition value with type detection
fn parse_condition_value(value_str: &str, operator: &Operator) -> Result<ConditionValue, RhemaError> {
    let value_str = value_str.trim();
    
    // Remove quotes if present
    let clean_value = if (value_str.starts_with("'") && value_str.ends_with("'")) ||
                         (value_str.starts_with("\"") && value_str.ends_with("\"")) {
        &value_str[1..value_str.len()-1]
    } else {
        value_str
    };
    
    match operator {
        Operator::Like | Operator::NotLike => {
            // Convert LIKE pattern to regex
            // Regex variant removed, convert to String for now
            Ok(ConditionValue::String(clean_value.to_string()))
        }
        Operator::In | Operator::NotIn => {
            // Parse array values
            let values: Vec<&str> = clean_value.split(',').map(|s| s.trim()).collect();
            let mut parsed_values = Vec::new();
            
            for val in values {
                let clean_val = if (val.starts_with("'") && val.ends_with("'")) ||
                                   (val.starts_with("\"") && val.ends_with("\"")) {
                    &val[1..val.len()-1]
                } else {
                    val
                };
                parsed_values.push(parse_single_value(clean_val)?);
            }
            
            Ok(ConditionValue::Array(parsed_values))
        }
        _ => {
            parse_single_value(clean_value)
        }
    }
}

/// Parse a single value with type detection
fn parse_single_value(value: &str) -> Result<ConditionValue, RhemaError> {
    match value.to_lowercase().as_str() {
        "true" => Ok(ConditionValue::Boolean(true)),
        "false" => Ok(ConditionValue::Boolean(false)),
        "null" => Ok(ConditionValue::Null),
        _ => {
            // Try to parse as number
            if let Ok(num) = value.parse::<f64>() {
                Ok(ConditionValue::Number(num))
            } else {
                // Try to parse as datetime
                if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
                    Ok(ConditionValue::DateTime(dt.with_timezone(&Utc)))
                } else if let Ok(ndt) = NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S") {
                    Ok(ConditionValue::DateTime(DateTime::from_naive_utc_and_offset(ndt, Utc)))
                } else {
                    // Default to string
                    Ok(ConditionValue::String(value.to_string()))
                }
            }
        }
    }
}

/// Parse ORDER BY clause
fn parse_order_by(order_by_clause: &str) -> Result<Option<Vec<OrderBy>>, RhemaError> {
    let parts: Vec<&str> = order_by_clause.split(',').collect();
    let mut order_by = Vec::new();
    
    for part in parts {
        let part = part.trim();
        let direction = if part.to_uppercase().ends_with(" DESC") {
            OrderDirection::Desc
        } else if part.to_uppercase().ends_with(" ASC") {
            OrderDirection::Asc
        } else {
            OrderDirection::Asc // Default
        };
        
        let field = if part.to_uppercase().ends_with(" DESC") {
            part[..part.len() - 5].trim()
        } else if part.to_uppercase().ends_with(" ASC") {
            part[..part.len() - 4].trim()
        } else {
            part
        };
        
        order_by.push(OrderBy {
            field: field.to_string(),
            direction,
        });
    }
    
    Ok(Some(order_by))
}

/// Check if a value matches the given conditions
fn matches_conditions(value: &Value, conditions: &[Condition]) -> Result<bool, RhemaError> {
    if conditions.is_empty() {
        return Ok(true);
    }
    
    let mut result = matches_condition(value, &conditions[0])?;
    
    for i in 1..conditions.len() {
        let condition = &conditions[i];
        let condition_result = matches_condition(value, condition)?;
        
        match condition.logical_op {
            LogicalOperator::And => {
                result = result && condition_result;
            }
            LogicalOperator::Or => {
                result = result || condition_result;
            }
            LogicalOperator::Not => {
                result = result && !condition_result;
            }
        }
    }
    
    Ok(result)
}

/// Extract field value from YAML
fn extract_field_value(value: &Value, field: &str) -> Result<Value, RhemaError> {
    match value {
        Value::Mapping(map) => {
            let key = Value::String(field.to_string());
            map.get(&key).cloned().ok_or_else(|| {
                RhemaError::InvalidQuery(format!("Field not found: {}", field))
            })
        }
        _ => Err(RhemaError::InvalidQuery(format!("Cannot extract field from non-object: {}", field)))
    }
}

/// Apply WHERE conditions to YAML data with enhanced filtering
fn apply_conditions(data: &Value, conditions: &[Condition]) -> Result<Value, RhemaError> {
    if conditions.is_empty() {
        return Ok(data.clone());
    }
    
    match data {
        Value::Sequence(seq) => {
            let mut filtered = Vec::new();
            
            for item in seq {
                if matches_conditions(item, conditions)? {
                    filtered.push(item.clone());
                }
            }
            
            Ok(Value::Sequence(filtered))
        }
        Value::Mapping(_map) => {
            if matches_conditions(data, conditions)? {
                Ok(data.clone())
            } else {
                Ok(Value::Null)
            }
        }
        _ => {
            if matches_conditions(data, conditions)? {
                Ok(data.clone())
            } else {
                Ok(Value::Null)
            }
        }
    }
}

/// Apply ORDER BY clause to results
fn apply_order_by(data: &Value, order_by: &[OrderBy]) -> Result<Value, RhemaError> {
    match data {
        Value::Sequence(seq) => {
            let mut sorted = seq.clone();
            
            for order_clause in order_by.iter().rev() {
                sorted.sort_by(|a, b| {
                    let a_val = extract_field_value(a, &order_clause.field).unwrap_or(Value::Null);
                    let b_val = extract_field_value(b, &order_clause.field).unwrap_or(Value::Null);
                    
                    let comparison = match (&a_val, &b_val) {
                        (Value::String(a_str), Value::String(b_str)) => a_str.cmp(b_str),
                        (Value::Number(a_num), Value::Number(b_num)) => {
                            if let (Some(a_f64), Some(b_f64)) = (a_num.as_f64(), b_num.as_f64()) {
                                a_f64.partial_cmp(&b_f64).unwrap_or(std::cmp::Ordering::Equal)
                            } else {
                                std::cmp::Ordering::Equal
                            }
                        }
                        (Value::Bool(a_bool), Value::Bool(b_bool)) => a_bool.cmp(b_bool),
                        _ => std::cmp::Ordering::Equal,
                    };
                    
                    match order_clause.direction {
                        OrderDirection::Asc => comparison,
                        OrderDirection::Desc => comparison.reverse(),
                    }
                });
            }
            
            Ok(Value::Sequence(sorted))
        }
        _ => Ok(data.clone()),
    }
}

/// Apply LIMIT and OFFSET to results
fn apply_limit_offset(data: &Value, limit: Option<usize>, offset: Option<usize>) -> Result<Value, RhemaError> {
    match data {
        Value::Sequence(seq) => {
            let mut result = seq.clone();
            
            // Apply offset
            if let Some(offset_val) = offset {
                if offset_val < result.len() {
                    result = result[offset_val..].to_vec();
                } else {
                    result.clear();
                }
            }
            
            // Apply limit
            if let Some(limit_val) = limit {
                if limit_val < result.len() {
                    result = result[..limit_val].to_vec();
                }
            }
            
            Ok(Value::Sequence(result))
        }
        _ => Ok(data.clone()),
    }
}

/// Execute a parsed query with enhanced features
fn execute_parsed_query(
    query: &CqlQuery,
    scopes: &[Scope],
    repo_root: &Path,
) -> Result<Vec<QueryResult>, RhemaError> {
    let mut results = Vec::new();
    
    // Determine which scopes to query
    let target_scopes = resolve_target_scopes(&query.target, scopes, repo_root)?;
    
    for scope in target_scopes {
        if let Some(file_path) = scope.get_file(&format!("{}.yaml", query.target)) {
            let content = std::fs::read_to_string(file_path)
                .map_err(|e| RhemaError::IoError(e))?;
            
            let yaml_data: Value = serde_yaml::from_str(&content)
                .map_err(|e| RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            
            // Apply YAML path if specified
            let mut filtered_data = if let Some(ref yaml_path) = query.yaml_path {
                extract_yaml_path(&yaml_data, yaml_path)?
            } else {
                yaml_data
            };
            
            // Apply WHERE conditions
            filtered_data = apply_conditions(&filtered_data, &query.conditions)?;
            
            // Apply ORDER BY if specified
            if let Some(ref order_by) = query.order_by {
                filtered_data = apply_order_by(&filtered_data, order_by)?;
            }
            
            // Apply LIMIT and OFFSET
            filtered_data = apply_limit_offset(&filtered_data, query.limit, query.offset)?;
            
            if !filtered_data.is_null() {
                let scope_rel_path = scope.relative_path(repo_root)?;
                results.push(QueryResult {
                    scope: scope_rel_path,
                    file: format!("{}.yaml", query.target),
                    data: filtered_data,
                    path: query.yaml_path.clone().unwrap_or_default(),
                    field_provenance: HashMap::new(),
                    query_provenance: None,
                    metadata: HashMap::new(),
                });
            }
        }
    }
    
    Ok(results)
}

/// Execute a parsed query with full provenance tracking
fn execute_parsed_query_with_provenance(
    query: &CqlQuery,
    scopes: &[Scope],
    repo_root: &Path,
    executed_at: &DateTime<Utc>,
) -> Result<Vec<QueryResult>, RhemaError> {
    let mut results = Vec::new();
    
    // Determine which scopes to query
    let target_scopes = resolve_target_scopes(&query.target, scopes, repo_root)?;
    
    for scope in target_scopes {
        if let Some(file_path) = scope.get_file(&format!("{}.yaml", query.target)) {
            let content = std::fs::read_to_string(file_path)
                .map_err(|e| RhemaError::IoError(e))?;
            
            let yaml_data: Value = serde_yaml::from_str(&content)
                .map_err(|e| RhemaError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            
            // Track field-level provenance
            let mut field_provenance = HashMap::new();
            
            // Apply YAML path if specified
            let mut filtered_data = if let Some(ref yaml_path) = query.yaml_path {
                let path_data = extract_yaml_path(&yaml_data, yaml_path)?;
                // Track field provenance for YAML path extraction
                track_field_provenance(&mut field_provenance, &yaml_data, &path_data, Some(yaml_path), scope, &query.target, executed_at)?;
                path_data
            } else {
                // Track field provenance for all fields
                track_field_provenance(&mut field_provenance, &yaml_data, &yaml_data, None, scope, &query.target, executed_at)?;
                yaml_data
            };
            
            // Apply WHERE conditions with provenance tracking
            let before_conditions = filtered_data.clone();
            filtered_data = apply_conditions(&filtered_data, &query.conditions)?;
            track_condition_provenance(&mut field_provenance, &before_conditions, &filtered_data, &query.conditions, executed_at)?;
            
            // Apply ORDER BY if specified
            if let Some(ref order_by) = query.order_by {
                let before_ordering = filtered_data.clone();
                filtered_data = apply_order_by(&filtered_data, order_by)?;
                track_ordering_provenance(&mut field_provenance, &before_ordering, &filtered_data, order_by, executed_at)?;
            }
            
            // Apply LIMIT and OFFSET
            let before_limit = filtered_data.clone();
            filtered_data = apply_limit_offset(&filtered_data, query.limit, query.offset)?;
            track_limit_provenance(&mut field_provenance, &before_limit, &filtered_data, query.limit, query.offset, executed_at)?;
            
            if !filtered_data.is_null() {
                let scope_rel_path = scope.relative_path(repo_root)?;
                results.push(QueryResult {
                    scope: scope_rel_path,
                    file: format!("{}.yaml", query.target),
                    data: filtered_data,
                    path: query.yaml_path.clone().unwrap_or_default(),
                    field_provenance,
                    query_provenance: None,
                    metadata: HashMap::new(),
                });
            }
        }
    }
    
    Ok(results)
}

/// Resolve target scopes based on query target
fn resolve_target_scopes<'a>(
    target: &str,
    scopes: &'a [Scope],
    _repo_root: &Path,
) -> Result<Vec<&'a Scope>, RhemaError> {
    // Handle wildcard patterns
    if target.contains('*') {
        return Ok(scopes.iter().collect());
    }
    
    // Handle relative paths
    if target.starts_with("../") || target.starts_with("./") {
        // For now, return all scopes - in a full implementation,
        // this would resolve relative paths based on current scope
        return Ok(scopes.iter().collect());
    }
    
    // Handle absolute paths
    if target.starts_with('/') {
        let target_path = PathBuf::from(target);
        let rhema_path = if target_path.file_name().and_then(|s| s.to_str()) == Some(".rhema") {
            target_path
        } else {
            target_path.join(".rhema")
        };
        
        for scope in scopes {
            if scope.path == rhema_path {
                return Ok(vec![scope]);
            }
        }
        
        return Err(RhemaError::ScopeNotFound(format!("Scope not found: {}", target)));
    }
    
    // Default: return all scopes that have the target file
    Ok(scopes.iter().filter(|scope| {
        scope.has_file(&format!("{}.yaml", target))
    }).collect())
}

/// Extract data from YAML using a path
pub fn extract_yaml_path(data: &Value, path: &str) -> Result<Value, RhemaError> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = data;
    
    for part in parts {
        match current {
            Value::Mapping(map) => {
                let key = Value::String(part.to_string());
                current = map.get(&key).ok_or_else(|| {
                    RhemaError::InvalidQuery(format!("Path not found: {}", path))
                })?;
            }
            Value::Sequence(seq) => {
                let index: usize = part.parse().map_err(|_| {
                    RhemaError::InvalidQuery(format!("Invalid array index: {}", part))
                })?;
                current = seq.get(index).ok_or_else(|| {
                    RhemaError::InvalidQuery(format!("Array index out of bounds: {}", index))
                })?;
            }
            _ => {
                return Err(RhemaError::InvalidQuery(format!("Cannot traverse path: {}", path)));
            }
        }
    }
    
    Ok(current.clone())
}

/// Check if a value matches a single condition
fn matches_condition(value: &Value, condition: &Condition) -> Result<bool, RhemaError> {
    // First extract the field value if this is a field-based condition
    let field_value = if !condition.field.is_empty() {
        extract_field_value(value, &condition.field)?
    } else {
        value.clone()
    };
    
    match condition.operator {
        Operator::Equals => {
            let condition_value = convert_condition_value_to_yaml(&condition.value)?;
            Ok(&field_value == &condition_value)
        }
        Operator::NotEquals => {
            let condition_value = convert_condition_value_to_yaml(&condition.value)?;
            Ok(&field_value != &condition_value)
        }
        Operator::GreaterThan => {
            let condition_value = convert_condition_value_to_yaml(&condition.value)?;
            Ok(&field_value > &condition_value)
        }
        Operator::LessThan => {
            let condition_value = convert_condition_value_to_yaml(&condition.value)?;
            Ok(&field_value < &condition_value)
        }
        Operator::GreaterThanOrEqual => {
            let condition_value = convert_condition_value_to_yaml(&condition.value)?;
            Ok(&field_value >= &condition_value)
        }
        Operator::LessThanOrEqual => {
            let condition_value = convert_condition_value_to_yaml(&condition.value)?;
            Ok(&field_value <= &condition_value)
        }
        Operator::Like => {
            match &condition.value {
                ConditionValue::String(pattern) => {
                    let regex_pattern = pattern.replace("%", ".*").replace("_", ".");
                    let regex = Regex::new(&format!("^{}$", regex_pattern))
                        .map_err(|_| RhemaError::InvalidQuery("Invalid LIKE pattern".to_string()))?;
                    let value_str = field_value.as_str().unwrap_or_default();
                    Ok(regex.is_match(value_str))
                }
                _ => Err(RhemaError::InvalidQuery("LIKE operator requires string value".to_string())),
            }
        }
        Operator::NotLike => {
            match &condition.value {
                ConditionValue::String(pattern) => {
                    let regex_pattern = pattern.replace("%", ".*").replace("_", ".");
                    let regex = Regex::new(&format!("^{}$", regex_pattern))
                        .map_err(|_| RhemaError::InvalidQuery("Invalid LIKE pattern".to_string()))?;
                    let value_str = field_value.as_str().unwrap_or_default();
                    Ok(!regex.is_match(value_str))
                }
                _ => Err(RhemaError::InvalidQuery("NOT LIKE operator requires string value".to_string())),
            }
        }
        Operator::In => {
            match &condition.value {
                ConditionValue::Array(arr) => {
                    for item in arr {
                        let item_value = convert_condition_value_to_yaml(item)?;
                        if &field_value == &item_value {
                            return Ok(true);
                        }
                    }
                    Ok(false)
                }
                _ => Err(RhemaError::InvalidQuery("IN operator requires array value".to_string())),
            }
        }
        Operator::NotIn => {
            match &condition.value {
                ConditionValue::Array(arr) => {
                    for item in arr {
                        let item_value = convert_condition_value_to_yaml(item)?;
                        if &field_value == &item_value {
                            return Ok(false);
                        }
                    }
                    Ok(true)
                }
                _ => Err(RhemaError::InvalidQuery("NOT IN operator requires array value".to_string())),
            }
        }
        Operator::Contains => {
            match &condition.value {
                ConditionValue::Array(arr) => {
                    if let Value::Sequence(seq) = &field_value {
                        for item in arr {
                            let item_value = convert_condition_value_to_yaml(item)?;
                            if seq.contains(&item_value) {
                                return Ok(true);
                            }
                        }
                    }
                    Ok(false)
                }
                _ => Err(RhemaError::InvalidQuery("CONTAINS operator requires array value".to_string())),
            }
        }
        Operator::NotContains => {
            match &condition.value {
                ConditionValue::Array(arr) => {
                    if let Value::Sequence(seq) = &field_value {
                        for item in arr {
                            let item_value = convert_condition_value_to_yaml(item)?;
                            if seq.contains(&item_value) {
                                return Ok(false);
                            }
                        }
                    }
                    Ok(true)
                }
                _ => Err(RhemaError::InvalidQuery("NOT CONTAINS operator requires array value".to_string())),
            }
        }
        Operator::IsNull => {
            Ok(field_value.is_null())
        }
        Operator::IsNotNull => {
            Ok(!field_value.is_null())
        }
    }
}

/// Convert ConditionValue to serde_yaml::Value for comparison
fn convert_condition_value_to_yaml(condition_value: &ConditionValue) -> Result<Value, RhemaError> {
    match condition_value {
        ConditionValue::String(s) => Ok(Value::String(s.clone())),
        ConditionValue::Number(n) => Ok(Value::Number(serde_yaml::Number::from(*n))),
        ConditionValue::Boolean(b) => Ok(Value::Bool(*b)),
        ConditionValue::Null => Ok(Value::Null),
        ConditionValue::Array(arr) => {
            let mut array_value = Vec::new();
            for item in arr {
                array_value.push(convert_condition_value_to_yaml(item)?);
            }
            Ok(Value::Sequence(array_value))
        }
        // Regex variant removed
        ConditionValue::DateTime(dt) => Ok(Value::String(dt.to_rfc3339())),
    }
}

/// Search across all context files
pub fn search_context(repo_root: &Path, term: &str, file_filter: Option<&str>) -> Result<Vec<QueryResult>, RhemaError> {
    let scopes = crate::scope::discover_scopes(repo_root)?;
    let mut results = Vec::new();
    
    for scope in &scopes {
        for (filename, file_path) in &scope.files {
            // Apply file filter if specified
            if let Some(filter) = file_filter {
                if !filename.contains(filter) {
                    continue;
                }
            }
            
            let content = std::fs::read_to_string(file_path)
                .map_err(|e| RhemaError::IoError(e))?;
            
            // Simple text search
            if content.to_lowercase().contains(&term.to_lowercase()) {
                let yaml_data: Value = serde_yaml::from_str(&content)
                    .map_err(|e| RhemaError::InvalidYaml {
                        file: file_path.display().to_string(),
                        message: e.to_string(),
                    })?;
                
                let scope_rel_path = scope.relative_path(repo_root)?;
                results.push(QueryResult {
                    scope: scope_rel_path,
                    file: filename.clone(),
                    data: yaml_data,
                    path: "".to_string(),
                    field_provenance: HashMap::new(),
                    query_provenance: None,
                    metadata: HashMap::new(),
                });
            }
        }
    }
    
    Ok(results)
}

/// Enhanced search with regex support
pub fn search_context_regex(repo_root: &Path, pattern: &str, file_filter: Option<&str>) -> Result<Vec<QueryResult>, RhemaError> {
    let regex = Regex::new(pattern)
        .map_err(|_| RhemaError::InvalidQuery(format!("Invalid regex pattern: {}", pattern)))?;
    
    let scopes = crate::scope::discover_scopes(repo_root)?;
    let mut results = Vec::new();
    
    for scope in &scopes {
        for (filename, file_path) in &scope.files {
            // Apply file filter if specified
            if let Some(filter) = file_filter {
                if !filename.contains(filter) {
                    continue;
                }
            }
            
            let content = std::fs::read_to_string(file_path)
                .map_err(|e| RhemaError::IoError(e))?;
            
            // Regex search
            if regex.is_match(&content) {
                let yaml_data: Value = serde_yaml::from_str(&content)
                    .map_err(|e| RhemaError::InvalidYaml {
                        file: file_path.display().to_string(),
                        message: e.to_string(),
                    })?;
                
                let scope_rel_path = scope.relative_path(repo_root)?;
                results.push(QueryResult {
                    scope: scope_rel_path,
                    file: filename.clone(),
                    data: yaml_data,
                    path: "".to_string(),
                    field_provenance: HashMap::new(),
                    query_provenance: None,
                    metadata: HashMap::new(),
                });
            }
        }
    }
    
    Ok(results)
}

/// Get query statistics (count, min, max, etc.)
pub fn get_query_stats(repo_root: &Path, query: &str) -> Result<HashMap<String, Value>, RhemaError> {
    let result = execute_query(repo_root, query)?;
    let mut stats = HashMap::new();
    
    match result {
        Value::Sequence(seq) => {
            stats.insert("count".to_string(), Value::Number(serde_yaml::Number::from(seq.len())));
            
            if !seq.is_empty() {
                // Calculate numeric stats if applicable
                let mut numeric_values = Vec::new();
                for item in &seq {
                    if let Some(num) = item.as_f64() {
                        numeric_values.push(num);
                    }
                }
                
                if !numeric_values.is_empty() {
                    numeric_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    stats.insert("min".to_string(), Value::Number(serde_yaml::Number::from(numeric_values[0])));
                    stats.insert("max".to_string(), Value::Number(serde_yaml::Number::from(numeric_values[numeric_values.len() - 1])));
                    
                    let sum: f64 = numeric_values.iter().sum();
                    let avg = sum / numeric_values.len() as f64;
                    stats.insert("sum".to_string(), Value::Number(serde_yaml::Number::from(sum)));
                    stats.insert("avg".to_string(), Value::Number(serde_yaml::Number::from(avg)));
                }
            }
        }
        _ => {
            stats.insert("count".to_string(), Value::Number(serde_yaml::Number::from(1)));
        }
    }
    
    Ok(stats)
} 

/// Track field-level provenance for YAML path extraction
fn track_field_provenance(
    field_provenance: &mut HashMap<String, FieldProvenance>,
    _original_data: &Value,
    extracted_data: &Value,
    yaml_path: Option<&String>,
    scope: &Scope,
    target_file: &str,
    executed_at: &DateTime<Utc>,
) -> Result<(), RhemaError> {
    let scope_rel_path = scope.relative_path(&scope.path.parent().unwrap_or(&scope.path))?;
    
    // Extract all fields from the data
    extract_fields_recursive(extracted_data, "", field_provenance, &scope_rel_path, target_file, yaml_path, executed_at)?;
    
    Ok(())
}

/// Extract fields recursively and track their provenance
fn extract_fields_recursive(
    data: &Value,
    current_path: &str,
    field_provenance: &mut HashMap<String, FieldProvenance>,
    scope_path: &str,
    source_file: &str,
    yaml_path: Option<&String>,
    executed_at: &DateTime<Utc>,
) -> Result<(), RhemaError> {
    match data {
        Value::Mapping(map) => {
            for (key, value) in map {
                let key_str = key.as_str().unwrap_or("unknown");
                let field_path = if current_path.is_empty() {
                    key_str.to_string()
                } else {
                    format!("{}.{}", current_path, key_str)
                };
                
                // Create field provenance
                let field_prov = FieldProvenance {
                    field_path: field_path.clone(),
                    source_scope: scope_path.to_string(),
                    source_file: source_file.to_string(),
                    source_yaml_path: yaml_path.cloned(),
                    data_type: get_value_type(value),
                    was_transformed: false,
                    transformations: Vec::new(),
                    confidence: 1.0,
                    last_modified: None,
                    metadata: HashMap::new(),
                };
                
                field_provenance.insert(field_path.clone(), field_prov);
                
                // Recursively process nested fields
                extract_fields_recursive(value, &field_path, field_provenance, scope_path, source_file, yaml_path, executed_at)?;
            }
        }
        Value::Sequence(seq) => {
            for (i, value) in seq.iter().enumerate() {
                let field_path = if current_path.is_empty() {
                    format!("[{}]", i)
                } else {
                    format!("{}[{}]", current_path, i)
                };
                
                // Create field provenance for array elements
                let field_prov = FieldProvenance {
                    field_path: field_path.clone(),
                    source_scope: scope_path.to_string(),
                    source_file: source_file.to_string(),
                    source_yaml_path: yaml_path.cloned(),
                    data_type: get_value_type(value),
                    was_transformed: false,
                    transformations: Vec::new(),
                    confidence: 1.0,
                    last_modified: None,
                    metadata: HashMap::new(),
                };
                
                field_provenance.insert(field_path.clone(), field_prov);
                
                // Recursively process nested fields
                extract_fields_recursive(value, &field_path, field_provenance, scope_path, source_file, yaml_path, executed_at)?;
            }
        }
        _ => {
            // For primitive values, create field provenance if not already exists
            if !current_path.is_empty() && !field_provenance.contains_key(current_path) {
                let field_prov = FieldProvenance {
                    field_path: current_path.to_string(),
                    source_scope: scope_path.to_string(),
                    source_file: source_file.to_string(),
                    source_yaml_path: yaml_path.cloned(),
                    data_type: get_value_type(data),
                    was_transformed: false,
                    transformations: Vec::new(),
                    confidence: 1.0,
                    last_modified: None,
                    metadata: HashMap::new(),
                };
                
                field_provenance.insert(current_path.to_string(), field_prov);
            }
        }
    }
    
    Ok(())
}

/// Get the type of a YAML value as a string
fn get_value_type(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Sequence(_) => "array".to_string(),
        Value::Mapping(_) => "object".to_string(),
        &Value::Tagged(_) => "tagged".to_string(),
    }
}

/// Track provenance for condition filtering
fn track_condition_provenance(
    field_provenance: &mut HashMap<String, FieldProvenance>,
    before_data: &Value,
    after_data: &Value,
    conditions: &[Condition],
    executed_at: &DateTime<Utc>,
) -> Result<(), RhemaError> {
    // Count items before and after filtering
    let items_before = count_items(before_data);
    let items_after = count_items(after_data);
    
    // Add transformation to relevant fields
    for condition in conditions {
        if let Some(field_prov) = field_provenance.get_mut(&condition.field) {
            let transformation = Transformation {
                transformation_type: TransformationType::ValueFiltering,
                description: format!("Filtered by condition: {} {} {:?}", 
                    condition.field, 
                    operator_to_string(&condition.operator), 
                    condition.value),
                applied_at: *executed_at,
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("operator".to_string(), Value::String(operator_to_string(&condition.operator)));
                    params.insert("value".to_string(), convert_condition_value_to_yaml(&condition.value)?);
                    params.insert("items_before".to_string(), Value::Number(items_before.into()));
                    params.insert("items_after".to_string(), Value::Number(items_after.into()));
                    params
                },
                input_value: Some(before_data.clone()),
                output_value: Some(after_data.clone()),
            };
            
            field_prov.transformations.push(transformation);
            field_prov.was_transformed = true;
        }
    }
    
    Ok(())
}

/// Track provenance for ordering operations
fn track_ordering_provenance(
    field_provenance: &mut HashMap<String, FieldProvenance>,
    before_data: &Value,
    after_data: &Value,
    order_by: &[OrderBy],
    executed_at: &DateTime<Utc>,
) -> Result<(), RhemaError> {
    for order in order_by {
        if let Some(field_prov) = field_provenance.get_mut(&order.field) {
            let transformation = Transformation {
                transformation_type: TransformationType::Sorting,
                description: format!("Sorted by {} in {} order", 
                    order.field, 
                    match order.direction {
                        OrderDirection::Asc => "ascending",
                        OrderDirection::Desc => "descending",
                    }),
                applied_at: *executed_at,
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("direction".to_string(), Value::String(match order.direction {
                        OrderDirection::Asc => "asc".to_string(),
                        OrderDirection::Desc => "desc".to_string(),
                    }));
                    params
                },
                input_value: Some(before_data.clone()),
                output_value: Some(after_data.clone()),
            };
            
            field_prov.transformations.push(transformation);
            field_prov.was_transformed = true;
        }
    }
    
    Ok(())
}

/// Track provenance for limit/offset operations
fn track_limit_provenance(
    field_provenance: &mut HashMap<String, FieldProvenance>,
    before_data: &Value,
    after_data: &Value,
    limit: Option<usize>,
    offset: Option<usize>,
    executed_at: &DateTime<Utc>,
) -> Result<(), RhemaError> {
    let items_before = count_items(before_data);
    let items_after = count_items(after_data);
    
    // Add transformation to all fields since this affects the entire result set
    for field_prov in field_provenance.values_mut() {
        let transformation = Transformation {
            transformation_type: TransformationType::ValueFiltering,
            description: format!("Applied limit/offset: limit={:?}, offset={:?}", limit, offset),
            applied_at: *executed_at,
            parameters: {
                let mut params = HashMap::new();
                if let Some(l) = limit {
                    params.insert("limit".to_string(), Value::Number(l.into()));
                }
                if let Some(o) = offset {
                    params.insert("offset".to_string(), Value::Number(o.into()));
                }
                params.insert("items_before".to_string(), Value::Number(items_before.into()));
                params.insert("items_after".to_string(), Value::Number(items_after.into()));
                params
            },
            input_value: Some(before_data.clone()),
            output_value: Some(after_data.clone()),
        };
        
        field_prov.transformations.push(transformation);
        field_prov.was_transformed = true;
    }
    
    Ok(())
}

/// Count items in a YAML value
fn count_items(value: &Value) -> usize {
    match value {
        Value::Sequence(seq) => seq.len(),
        Value::Mapping(map) => map.len(),
        _ => 1,
    }
}

/// Convert operator to string representation
fn operator_to_string(operator: &Operator) -> String {
    match operator {
        Operator::Equals => "=".to_string(),
        Operator::NotEquals => "!=".to_string(),
        Operator::GreaterThan => ">".to_string(),
        Operator::LessThan => "<".to_string(),
        Operator::GreaterThanOrEqual => ">=".to_string(),
        Operator::LessThanOrEqual => "<=".to_string(),
        Operator::Like => "LIKE".to_string(),
        Operator::NotLike => "NOT LIKE".to_string(),
        Operator::In => "IN".to_string(),
        Operator::NotIn => "NOT IN".to_string(),
        Operator::Contains => "CONTAINS".to_string(),
        Operator::NotContains => "NOT CONTAINS".to_string(),
        Operator::IsNull => "IS NULL".to_string(),
        Operator::IsNotNull => "IS NOT NULL".to_string(),
    }
}

/// Build comprehensive query provenance information
fn build_query_provenance(
    original_query: &str,
    parsed_query: &CqlQuery,
    executed_at: DateTime<Utc>,
    total_duration: u64,
    scopes: &[Scope],
    results: &[QueryResult],
    parse_duration: u64,
    scope_duration: u64,
) -> Result<QueryProvenance, RhemaError> {
    let mut execution_steps = Vec::new();
    let mut applied_filters = Vec::new();
    let mut phase_times = HashMap::new();
    
    // Add execution steps
    execution_steps.push(ExecutionStep {
        name: "Query Parsing".to_string(),
        step_type: ExecutionStepType::QueryParsing,
        duration_ms: parse_duration,
        input_size: Some(original_query.len()),
        output_size: None,
        metadata: HashMap::new(),
    });
    
    execution_steps.push(ExecutionStep {
        name: "Scope Discovery".to_string(),
        step_type: ExecutionStepType::ScopeResolution,
        duration_ms: scope_duration,
        input_size: None,
        output_size: Some(scopes.len()),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("scopes_found".to_string(), Value::Number(scopes.len().into()));
            meta
        },
    });
    
    // Add phase times
    phase_times.insert("parsing".to_string(), parse_duration);
    phase_times.insert("scope_discovery".to_string(), scope_duration);
    phase_times.insert("execution".to_string(), total_duration - parse_duration - scope_duration);
    
    // Build applied filters
    if !parsed_query.conditions.is_empty() {
        applied_filters.push(AppliedFilter {
            filter_type: FilterType::WhereCondition,
            description: format!("Applied {} WHERE conditions", parsed_query.conditions.len()),
            items_before: 0, // Would need to track this during execution
            items_after: results.len(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("condition_count".to_string(), Value::Number(parsed_query.conditions.len().into()));
                params
            },
        });
    }
    
    if parsed_query.yaml_path.is_some() {
        applied_filters.push(AppliedFilter {
            filter_type: FilterType::YamlPath,
            description: format!("Applied YAML path: {}", parsed_query.yaml_path.as_ref().unwrap()),
            items_before: 0,
            items_after: results.len(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("yaml_path".to_string(), Value::String(parsed_query.yaml_path.as_ref().unwrap().clone()));
                params
            },
        });
    }
    
    if let Some(ref order_by) = parsed_query.order_by {
        applied_filters.push(AppliedFilter {
            filter_type: FilterType::OrderBy,
            description: format!("Applied ORDER BY with {} fields", order_by.len()),
            items_before: 0,
            items_after: results.len(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("order_fields".to_string(), Value::Number(order_by.len().into()));
                params
            },
        });
    }
    
    if parsed_query.limit.is_some() || parsed_query.offset.is_some() {
        applied_filters.push(AppliedFilter {
            filter_type: FilterType::Limit,
            description: format!("Applied LIMIT={:?} OFFSET={:?}", parsed_query.limit, parsed_query.offset),
            items_before: 0,
            items_after: results.len(),
            parameters: {
                let mut params = HashMap::new();
                if let Some(l) = parsed_query.limit {
                    params.insert("limit".to_string(), Value::Number(l.into()));
                }
                if let Some(o) = parsed_query.offset {
                    params.insert("offset".to_string(), Value::Number(o.into()));
                }
                params
            },
        });
    }
    
    // Build performance metrics
    let performance_metrics = PerformanceMetrics {
        total_time_ms: total_duration,
        phase_times,
        memory_usage_bytes: None, // Would need to implement memory tracking
        files_read: results.len(),
        yaml_documents_processed: results.len(),
        cache_stats: None, // Would need to implement caching
    };
    
    Ok(QueryProvenance {
        original_query: original_query.to_string(),
        parsed_query: parsed_query.clone(),
        executed_at,
        execution_time_ms: total_duration,
        scopes_searched: scopes.iter().map(|s| s.definition.name.clone()).collect(),
        files_accessed: results.iter().map(|r| r.file.clone()).collect(),
        execution_steps,
        applied_filters,
        performance_metrics,
        errors: None,
    })
} 