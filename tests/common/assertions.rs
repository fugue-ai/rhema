//! Common assertions for Rhema CLI tests

use serde_yaml::Value;
use std::path::Path;

/// Assert that a query result contains expected content
#[allow(dead_code)]
pub fn assert_query_contains(result: &Value, expected: &str) {
    let result_str = serde_yaml::to_string(result).unwrap();
    assert!(
        result_str.contains(expected),
        "Query result should contain '{}', but got: {}",
        expected,
        result_str
    );
}

/// Assert that a file exists
#[allow(dead_code)]
pub fn assert_file_exists(path: &Path) {
    assert!(
        path.exists() && path.is_file(),
        "File should exist: {}",
        path.display()
    );
}

/// Assert that a directory exists
#[allow(dead_code)]
pub fn assert_dir_exists(path: &Path) {
    assert!(
        path.exists() && path.is_dir(),
        "Directory should exist: {}",
        path.display()
    );
}

/// Assert that a query result is not empty
#[allow(dead_code)]
pub fn assert_query_not_empty(result: &Value) {
    match result {
        Value::Sequence(seq) => {
            assert!(!seq.is_empty(), "Query result should not be empty");
        }
        Value::Mapping(map) => {
            assert!(!map.is_empty(), "Query result should not be empty");
        }
        Value::Null => {
            panic!("Query result should not be null");
        }
        _ => {
            // For scalar values, just check it's not null
            assert!(!result.is_null(), "Query result should not be null");
        }
    }
}

/// Assert that a query result has the expected length
#[allow(dead_code)]
pub fn assert_query_length(result: &Value, expected_len: usize) {
    match result {
        Value::Sequence(seq) => {
            assert_eq!(
                seq.len(),
                expected_len,
                "Query result should have {} items, but got {}",
                expected_len,
                seq.len()
            );
        }
        _ => {
            panic!("Cannot check length of non-sequence result");
        }
    }
}

/// Assert that a query result has a specific key
#[allow(dead_code)]
pub fn assert_query_has_key(result: &Value, key: &str) {
    match result {
        Value::Mapping(map) => {
            let key_value = Value::String(key.to_string());
            assert!(
                map.contains_key(&key_value),
                "Query result should contain key '{}'",
                key
            );
        }
        _ => {
            panic!("Cannot check keys of non-mapping result");
        }
    }
}

/// Assert that a query result has a specific value
#[allow(dead_code)]
pub fn assert_query_has_value(result: &Value, expected_value: &str) {
    let result_str = serde_yaml::to_string(result).unwrap();
    assert!(
        result_str.contains(expected_value),
        "Query result should contain value '{}', but got: {}",
        expected_value,
        result_str
    );
}

/// Assert that execution time is within acceptable limits
#[allow(dead_code)]
pub fn assert_execution_time(time_ms: u64, max_time_ms: u64) {
    assert!(
        time_ms <= max_time_ms,
        "Execution time {}ms exceeds maximum {}ms",
        time_ms,
        max_time_ms
    );
}

/// Assert that a Git repository is properly initialized
#[allow(dead_code)]
pub fn assert_git_repo_initialized(path: &Path) {
    let git_dir = path.join(".git");
    assert!(
        git_dir.exists() && git_dir.is_dir(),
        "Git repository should be initialized at {}",
        path.display()
    );
}

/// Assert that Rhema structure is properly set up
#[allow(dead_code)]
pub fn assert_rhema_structure(path: &Path) {
    let rhema_dir = path.join(".rhema");
    assert!(
        rhema_dir.exists() && rhema_dir.is_dir(),
        "Rhema directory should exist at {}",
        path.display()
    );

    let rhema_yaml = rhema_dir.join("rhema.yaml");
    assert!(
        rhema_yaml.exists() && rhema_yaml.is_file(),
        "rhema.yaml should exist in Rhema directory"
    );
}

/// Assert that YAML content is valid
#[allow(dead_code)]
pub fn assert_valid_scope_yaml(content: &str) {
    let result: Result<Value, _> = serde_yaml::from_str(content);
    assert!(
        result.is_ok(),
        "YAML content should be valid: {}",
        result.unwrap_err()
    );
}

/// Assert that search results contain expected patterns
#[allow(dead_code)]
pub fn assert_search_results_contain(results: &[String], expected_pattern: &str) {
    let found = results
        .iter()
        .any(|result| result.contains(expected_pattern));
    assert!(
        found,
        "Search results should contain pattern '{}', but got: {:?}",
        expected_pattern, results
    );
}

/// Assert that an error contains expected content
#[allow(dead_code)]
pub fn assert_error_contains(error: &dyn std::error::Error, expected: &str) {
    let error_msg = error.to_string();
    assert!(
        error_msg.contains(expected),
        "Error should contain '{}', but got: {}",
        expected,
        error_msg
    );
}

/// Assert that a value has valid structure
#[allow(dead_code)]
pub fn assert_valid_structure(value: &Value) {
    // Basic validation - ensure it's not null and can be serialized
    assert!(!value.is_null(), "Value should not be null");

    let serialized = serde_yaml::to_string(value);
    assert!(
        serialized.is_ok(),
        "Value should be serializable: {}",
        serialized.unwrap_err()
    );
}
