use crate::{GacpError, scope::Scope};
use serde_yaml::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use regex::Regex;

/// CQL query structure
#[derive(Debug, Clone)]
pub struct CqlQuery {
    /// Target file or path
    pub target: String,
    
    /// YAML path within the file
    pub yaml_path: Option<String>,
    
    /// WHERE clause conditions
    pub conditions: Vec<Condition>,
    
    /// Scope context for relative paths
    pub scope_context: Option<String>,
}

/// Query condition
#[derive(Debug, Clone)]
pub struct Condition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

/// Query result with metadata
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub scope: String,
    pub file: String,
    pub data: Value,
    pub path: String,
}

/// Execute a CQL query
pub fn execute_query(repo_root: &Path, query: &str) -> Result<Value, GacpError> {
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

/// Parse a CQL query string
pub fn parse_cql_query(query: &str) -> Result<CqlQuery, GacpError> {
    let query = query.trim();
    
    // Simple regex-based parser for basic CQL syntax
    let re = Regex::new(r"^([^\s]+)(?:\s+WHERE\s+(.+))?$").map_err(|_| {
        GacpError::InvalidQuery("Invalid regex pattern".to_string())
    })?;
    
    let captures = re.captures(query).ok_or_else(|| {
        GacpError::InvalidQuery(format!("Invalid query syntax: {}", query))
    })?;
    
    let target = captures[1].to_string();
    let where_clause = captures.get(2).map(|m| m.as_str().to_string());
    
    // Parse target into file and yaml_path
    let (_file, yaml_path) = parse_target(&target)?;
    
    // Parse WHERE conditions
    let conditions = if let Some(where_clause) = where_clause {
        parse_conditions(&where_clause)?
    } else {
        Vec::new()
    };
    
    Ok(CqlQuery {
        target,
        yaml_path,
        conditions,
        scope_context: None,
    })
}

/// Parse target into file and YAML path
fn parse_target(target: &str) -> Result<(String, Option<String>), GacpError> {
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

/// Parse WHERE conditions
fn parse_conditions(where_clause: &str) -> Result<Vec<Condition>, GacpError> {
    let mut conditions = Vec::new();
    
    // Split by AND (simple parsing)
    let parts: Vec<&str> = where_clause.split(" AND ").collect();
    
    for part in parts {
        let condition = parse_single_condition(part.trim())?;
        conditions.push(condition);
    }
    
    Ok(conditions)
}

/// Parse a single condition
fn parse_single_condition(condition: &str) -> Result<Condition, GacpError> {
    // Match patterns like "field='value'" or "field=value"
    let re = Regex::new(r"^([^=]+)=(.+)$").map_err(|_| {
        GacpError::InvalidQuery("Invalid condition regex".to_string())
    })?;
    
    let captures = re.captures(condition).ok_or_else(|| {
        GacpError::InvalidQuery(format!("Invalid condition syntax: {}", condition))
    })?;
    
    let value = captures[2].trim();
    let value = if value.starts_with("'") && value.ends_with("'") {
        &value[1..value.len()-1]
    } else if value.starts_with("\"") && value.ends_with("\"") {
        &value[1..value.len()-1]
    } else {
        value
    };
    
    Ok(Condition {
        field: captures[1].trim().to_string(),
        operator: "=".to_string(),
        value: value.to_string(),
    })
}

/// Execute a parsed query
fn execute_parsed_query(
    query: &CqlQuery,
    scopes: &[Scope],
    repo_root: &Path,
) -> Result<Vec<QueryResult>, GacpError> {
    let mut results = Vec::new();
    
    // Determine which scopes to query
    let target_scopes = resolve_target_scopes(&query.target, scopes, repo_root)?;
    
    for scope in target_scopes {
        if let Some(file_path) = scope.get_file(&format!("{}.yaml", query.target)) {
            let content = std::fs::read_to_string(file_path)
                .map_err(|e| GacpError::IoError(e))?;
            
            let yaml_data: Value = serde_yaml::from_str(&content)
                .map_err(|e| GacpError::InvalidYaml {
                    file: file_path.display().to_string(),
                    message: e.to_string(),
                })?;
            
            // Apply YAML path if specified
            let filtered_data = if let Some(ref yaml_path) = query.yaml_path {
                extract_yaml_path(&yaml_data, yaml_path)?
            } else {
                yaml_data
            };
            
            // Apply WHERE conditions
            let filtered_data = apply_conditions(&filtered_data, &query.conditions)?;
            
            if !filtered_data.is_null() {
                let scope_rel_path = scope.relative_path(repo_root)?;
                results.push(QueryResult {
                    scope: scope_rel_path,
                    file: format!("{}.yaml", query.target),
                    data: filtered_data,
                    path: query.yaml_path.clone().unwrap_or_default(),
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
) -> Result<Vec<&'a Scope>, GacpError> {
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
        let gacp_path = if target_path.file_name().and_then(|s| s.to_str()) == Some(".gacp") {
            target_path
        } else {
            target_path.join(".gacp")
        };
        
        for scope in scopes {
            if scope.path == gacp_path {
                return Ok(vec![scope]);
            }
        }
        
        return Err(GacpError::ScopeNotFound(format!("Scope not found: {}", target)));
    }
    
    // Default: return all scopes that have the target file
    Ok(scopes.iter().filter(|scope| {
        scope.has_file(&format!("{}.yaml", target))
    }).collect())
}

/// Extract data from YAML using a path
fn extract_yaml_path(data: &Value, path: &str) -> Result<Value, GacpError> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = data;
    
    for part in parts {
        match current {
            Value::Mapping(map) => {
                let key = Value::String(part.to_string());
                current = map.get(&key).ok_or_else(|| {
                    GacpError::InvalidQuery(format!("Path not found: {}", path))
                })?;
            }
            Value::Sequence(seq) => {
                let index: usize = part.parse().map_err(|_| {
                    GacpError::InvalidQuery(format!("Invalid array index: {}", part))
                })?;
                current = seq.get(index).ok_or_else(|| {
                    GacpError::InvalidQuery(format!("Array index out of bounds: {}", index))
                })?;
            }
            _ => {
                return Err(GacpError::InvalidQuery(format!("Cannot traverse path: {}", path)));
            }
        }
    }
    
    Ok(current.clone())
}

/// Apply WHERE conditions to YAML data
fn apply_conditions(data: &Value, conditions: &[Condition]) -> Result<Value, GacpError> {
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

/// Check if a value matches the given conditions
fn matches_conditions(value: &Value, conditions: &[Condition]) -> Result<bool, GacpError> {
    for condition in conditions {
        let field_value = extract_field_value(value, &condition.field)?;
        
        if !matches_condition(&field_value, condition)? {
            return Ok(false);
        }
    }
    
    Ok(true)
}

/// Extract field value from YAML
fn extract_field_value(value: &Value, field: &str) -> Result<Value, GacpError> {
    match value {
        Value::Mapping(map) => {
            let key = Value::String(field.to_string());
            map.get(&key).cloned().ok_or_else(|| {
                GacpError::InvalidQuery(format!("Field not found: {}", field))
            })
        }
        _ => Err(GacpError::InvalidQuery(format!("Cannot extract field from non-object: {}", field)))
    }
}

/// Check if a value matches a single condition
fn matches_condition(value: &Value, condition: &Condition) -> Result<bool, GacpError> {
    match condition.operator.as_str() {
        "=" => {
            let condition_value = match condition.value.as_str() {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                "null" => Value::Null,
                _ => Value::String(condition.value.clone()),
            };
            Ok(value == &condition_value)
        }
        "!=" => {
            let condition_value = match condition.value.as_str() {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                "null" => Value::Null,
                _ => Value::String(condition.value.clone()),
            };
            Ok(value != &condition_value)
        }
        _ => Err(GacpError::InvalidQuery(format!("Unsupported operator: {}", condition.operator)))
    }
}

/// Search across all context files
pub fn search_context(repo_root: &Path, term: &str, file_filter: Option<&str>) -> Result<Vec<QueryResult>, GacpError> {
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
                .map_err(|e| GacpError::IoError(e))?;
            
            // Simple text search
            if content.to_lowercase().contains(&term.to_lowercase()) {
                let yaml_data: Value = serde_yaml::from_str(&content)
                    .map_err(|e| GacpError::InvalidYaml {
                        file: file_path.display().to_string(),
                        message: e.to_string(),
                    })?;
                
                let scope_rel_path = scope.relative_path(repo_root)?;
                results.push(QueryResult {
                    scope: scope_rel_path,
                    file: filename.clone(),
                    data: yaml_data,
                    path: "".to_string(),
                });
            }
        }
    }
    
    Ok(results)
} 