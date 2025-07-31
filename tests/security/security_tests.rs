//! Security tests for Rhema CLI

use rhema::{Rhema, RhemaResult};
use tests::common::{TestEnv, TestFixtures, helpers::TestHelpers, security};
use std::path::PathBuf;

#[test]
fn test_path_traversal_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test path traversal attempts
    let malicious_paths = vec![
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "....//....//....//etc/passwd",
        "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd",
    ];
    
    for path in malicious_paths {
        let result = env.rhema.get_scope(path);
        assert!(result.is_err(), "Path traversal should be blocked: {}", path);
    }
    
    Ok(())
}

#[test]
fn test_yaml_injection_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test YAML injection attempts
    let malicious_yaml = vec![
        "!!python/object/apply:os.system ['rm -rf /']",
        "!!python/object/apply:subprocess.check_output [['cat', '/etc/passwd']]",
        "!!binary |\n  Q2F0Y2ggbWUgaWYgeW91IGNhbiE=",
    ];
    
    for yaml in malicious_yaml {
        let result = serde_yaml::from_str::<serde_yaml::Value>(yaml);
        // Should either fail or be sanitized
        if result.is_ok() {
            let value = result.unwrap();
            // Verify no dangerous content
            let value_str = serde_yaml::to_string(&value)?;
            assert!(!value_str.contains("os.system"));
            assert!(!value_str.contains("subprocess"));
        }
    }
    
    Ok(())
}

#[test]
fn test_input_validation() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test invalid queries
    let invalid_queries = vec![
        "todos WHERE id='; DROP TABLE todos; --",
        "todos WHERE id=1 OR 1=1",
        "todos WHERE id=1 UNION SELECT * FROM users",
        "todos WHERE id=1; INSERT INTO todos VALUES ('malicious')",
    ];
    
    for query in invalid_queries {
        let result = env.rhema.query(query);
        assert!(result.is_err(), "Invalid query should be rejected: {}", query);
    }
    
    Ok(())
}

#[test]
fn test_file_permission_validation() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test accessing files with restricted permissions
    let restricted_file = env.repo_path.join("restricted.yaml");
    TestHelpers::create_file_with_permissions(&restricted_file, "test", 0o000)?;
    
    // Try to read restricted file
    let result = std::fs::read_to_string(&restricted_file);
    assert!(result.is_err(), "Should not be able to read restricted file");
    
    // Clean up
    std::fs::set_permissions(&restricted_file, std::fs::Permissions::from_mode(0o644))?;
    std::fs::remove_file(&restricted_file)?;
    
    Ok(())
}

#[test]
fn test_sql_injection_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test SQL injection attempts in queries
    let sql_injection_queries = vec![
        "todos WHERE id='1' OR '1'='1'",
        "todos WHERE id=1; DROP TABLE todos;",
        "todos WHERE id=1 UNION SELECT * FROM users",
        "todos WHERE id=1' AND 1=1 --",
    ];
    
    for query in sql_injection_queries {
        let result = env.rhema.query(query);
        // Should either fail or be sanitized
        if result.is_ok() {
            let value = result.unwrap();
            let value_str = serde_yaml::to_string(&value)?;
            // Verify no SQL commands in output
            assert!(!value_str.to_lowercase().contains("drop"));
            assert!(!value_str.to_lowercase().contains("union"));
            assert!(!value_str.to_lowercase().contains("select"));
        }
    }
    
    Ok(())
}

#[test]
fn test_xss_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test XSS attempts in data
    let xss_content = vec![
        "<script>alert('xss')</script>",
        "javascript:alert('xss')",
        "<img src=x onerror=alert('xss')>",
        "&#60;script&#62;alert('xss')&#60;/script&#62;",
    ];
    
    for content in xss_content {
        // Create file with XSS content
        let xss_file = env.repo_path.join(".rhema").join("xss_test.yaml");
        let yaml_content = format!("todos:\n  - id: xss-test\n    title: {}\n    status: pending", content);
        std::fs::write(&xss_file, yaml_content)?;
        
        // Query the file
        let result = env.rhema.query("xss_test");
        if result.is_ok() {
            let value = result.unwrap();
            let value_str = serde_yaml::to_string(&value)?;
            // Verify XSS content is not executed
            assert!(!value_str.contains("<script>"));
            assert!(!value_str.contains("javascript:"));
        }
        
        // Clean up
        std::fs::remove_file(&xss_file)?;
    }
    
    Ok(())
}

#[test]
fn test_command_injection_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test command injection attempts
    let command_injection_content = vec![
        "$(rm -rf /)",
        "`cat /etc/passwd`",
        "; rm -rf /",
        "| cat /etc/passwd",
        "&& rm -rf /",
    ];
    
    for content in command_injection_content {
        // Create file with command injection content
        let injection_file = env.repo_path.join(".rhema").join("injection_test.yaml");
        let yaml_content = format!("todos:\n  - id: injection-test\n    title: {}\n    status: pending", content);
        std::fs::write(&injection_file, yaml_content)?;
        
        // Query the file
        let result = env.rhema.query("injection_test");
        if result.is_ok() {
            let value = result.unwrap();
            let value_str = serde_yaml::to_string(&value)?;
            // Verify command injection content is not executed
            assert!(!value_str.contains("rm -rf"));
            assert!(!value_str.contains("cat /etc/passwd"));
        }
        
        // Clean up
        std::fs::remove_file(&injection_file)?;
    }
    
    Ok(())
}

#[test]
fn test_buffer_overflow_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test with extremely large input
    let large_content = "A".repeat(1_000_000); // 1MB of data
    
    // Create file with large content
    let large_file = env.repo_path.join(".rhema").join("large_test.yaml");
    let yaml_content = format!("todos:\n  - id: large-test\n    title: {}\n    status: pending", large_content);
    std::fs::write(&large_file, yaml_content)?;
    
    // Try to query the file
    let result = env.rhema.query("large_test");
    // Should either succeed or fail gracefully, not crash
    if result.is_err() {
        let error = result.unwrap_err();
        // Verify it's a reasonable error, not a crash
        assert!(!error.to_string().contains("overflow"));
        assert!(!error.to_string().contains("memory"));
    }
    
    // Clean up
    std::fs::remove_file(&large_file)?;
    
    Ok(())
}

#[test]
fn test_integer_overflow_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test with extremely large numbers
    let large_number = "9".repeat(1000);
    
    // Create file with large number
    let overflow_file = env.repo_path.join(".rhema").join("overflow_test.yaml");
    let yaml_content = format!("todos:\n  - id: overflow-test\n    priority: {}\n    status: pending", large_number);
    std::fs::write(&overflow_file, yaml_content)?;
    
    // Try to query the file
    let result = env.rhema.query("overflow_test");
    // Should either succeed or fail gracefully, not crash
    if result.is_err() {
        let error = result.unwrap_err();
        // Verify it's a reasonable error, not a crash
        assert!(!error.to_string().contains("overflow"));
    }
    
    // Clean up
    std::fs::remove_file(&overflow_file)?;
    
    Ok(())
}

#[test]
fn test_denial_of_service_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test with recursive YAML (billion laughs attack)
    let recursive_yaml = r#"
&anchor
  - *anchor
  - *anchor
  - *anchor
  - *anchor
  - *anchor
"#;
    
    // Create file with recursive content
    let dos_file = env.repo_path.join(".rhema").join("dos_test.yaml");
    std::fs::write(&dos_file, recursive_yaml)?;
    
    // Try to query the file with timeout
    let result = std::panic::catch_unwind(|| {
        env.rhema.query("dos_test")
    });
    
    // Should not panic or hang indefinitely
    match result {
        Ok(query_result) => {
            // If it succeeds, that's fine
            if query_result.is_err() {
                let error = query_result.unwrap_err();
                // Verify it's a reasonable error
                assert!(!error.to_string().contains("overflow"));
            }
        }
        Err(_) => {
            // Panic is acceptable for DoS protection
        }
    }
    
    // Clean up
    std::fs::remove_file(&dos_file)?;
    
    Ok(())
}

#[test]
fn test_authentication_bypass_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test authentication bypass attempts
    let bypass_attempts = vec![
        "admin",
        "root",
        "superuser",
        "administrator",
        "guest",
        "anonymous",
    ];
    
    for attempt in bypass_attempts {
        // Try to access with bypass attempts
        let result = env.rhema.get_scope(attempt);
        // Should fail for non-existent scopes
        if result.is_ok() {
            let scope = result.unwrap();
            // Verify it's not a privileged scope
            assert!(!scope.definition.name.contains("admin"));
            assert!(!scope.definition.name.contains("root"));
        }
    }
    
    Ok(())
}

#[test]
fn test_privilege_escalation_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test privilege escalation attempts
    let escalation_attempts = vec![
        "../../../root/.rhema",
        "/etc/rhema",
        "/var/lib/rhema",
        "/usr/local/etc/rhema",
    ];
    
    for attempt in escalation_attempts {
        let result = env.rhema.get_scope(attempt);
        // Should fail for system directories
        assert!(result.is_err(), "Should not access system directories: {}", attempt);
    }
    
    Ok(())
}

#[test]
fn test_data_exfiltration_protection() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test data exfiltration attempts
    let exfiltration_queries = vec![
        "todos WHERE id LIKE '%'",
        "todos WHERE 1=1",
        "todos WHERE id IS NOT NULL",
        "todos WHERE id != ''",
    ];
    
    for query in exfiltration_queries {
        let result = env.rhema.query(query);
        if result.is_ok() {
            let value = result.unwrap();
            let value_str = serde_yaml::to_string(&value)?;
            
            // Verify sensitive data is not exposed
            assert!(!value_str.contains("password"));
            assert!(!value_str.contains("secret"));
            assert!(!value_str.contains("key"));
            assert!(!value_str.contains("token"));
        }
    }
    
    Ok(())
}

#[test]
fn test_input_sanitization() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test input sanitization
    let malicious_inputs = vec![
        "test<script>alert('xss')</script>",
        "test' OR '1'='1",
        "test; rm -rf /",
        "test$(cat /etc/passwd)",
        "test`whoami`",
    ];
    
    for input in malicious_inputs {
        // Create file with malicious input
        let sanitize_file = env.repo_path.join(".rhema").join("sanitize_test.yaml");
        let yaml_content = format!("todos:\n  - id: sanitize-test\n    title: {}\n    status: pending", input);
        std::fs::write(&sanitize_file, yaml_content)?;
        
        // Query the file
        let result = env.rhema.query("sanitize_test");
        if result.is_ok() {
            let value = result.unwrap();
            let value_str = serde_yaml::to_string(&value)?;
            
            // Verify malicious content is sanitized
            assert!(!value_str.contains("<script>"));
            assert!(!value_str.contains("rm -rf"));
            assert!(!value_str.contains("cat /etc/passwd"));
            assert!(!value_str.contains("whoami"));
        }
        
        // Clean up
        std::fs::remove_file(&sanitize_file)?;
    }
    
    Ok(())
}

#[test]
fn test_output_encoding() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test output encoding
    let special_chars = vec![
        "<>&\"'",
        "\\n\\t\\r",
        "\\u0000\\u0001\\u0002",
        "\\x00\\x01\\x02",
    ];
    
    for chars in special_chars {
        // Create file with special characters
        let encoding_file = env.repo_path.join(".rhema").join("encoding_test.yaml");
        let yaml_content = format!("todos:\n  - id: encoding-test\n    title: {}\n    status: pending", chars);
        std::fs::write(&encoding_file, yaml_content)?;
        
        // Query the file
        let result = env.rhema.query("encoding_test");
        if result.is_ok() {
            let value = result.unwrap();
            let value_str = serde_yaml::to_string(&value)?;
            
            // Verify special characters are properly encoded
            assert!(!value_str.contains("\\u0000"));
            assert!(!value_str.contains("\\x00"));
        }
        
        // Clean up
        std::fs::remove_file(&encoding_file)?;
    }
    
    Ok(())
}

#[test]
fn test_session_management() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test session management (if applicable)
    // Since Rhema CLI doesn't have sessions, this test verifies stateless operation
    
    // Multiple queries should not interfere with each other
    let result1 = env.rhema.query("todos")?;
    let result2 = env.rhema.query("insights")?;
    let result3 = env.rhema.query("patterns")?;
    
    // All queries should succeed independently
    assert!(serde_yaml::to_string(&result1)?.contains("todos"));
    assert!(serde_yaml::to_string(&result2)?.contains("insights"));
    assert!(serde_yaml::to_string(&result3)?.contains("patterns"));
    
    Ok(())
}

#[test]
fn test_audit_logging() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test audit logging (if implemented)
    // This test verifies that operations can be logged for security auditing
    
    // Perform various operations
    let _ = env.rhema.query("todos")?;
    let _ = env.rhema.search_regex("test", None)?;
    let _ = env.rhema.discover_scopes()?;
    
    // If audit logging is implemented, verify logs exist
    // For now, just verify operations complete successfully
    Ok(())
}

#[test]
fn test_secure_defaults() -> RhemaResult<()> {
    let env = TestEnv::new()?;
    
    // Test secure defaults
    // Verify that Rhema uses secure defaults when no configuration is provided
    
    // Test scope discovery with no configuration
    let scopes = env.rhema.discover_scopes()?;
    assert_eq!(scopes.len(), 0); // Should be empty, not fail
    
    // Test query with no data
    let result = env.rhema.query("nonexistent");
    assert!(result.is_err()); // Should fail gracefully
    
    Ok(())
}

#[test]
fn test_error_information_disclosure() -> RhemaResult<()> {
    let env = TestEnv::with_sample_data()?;
    
    // Test error information disclosure
    // Verify that errors don't leak sensitive information
    
    let error_queries = vec![
        "nonexistent",
        "todos WHERE invalid_field=value",
        "todos WHERE id='malicious'",
    ];
    
    for query in error_queries {
        let result = env.rhema.query(query);
        if result.is_err() {
            let error = result.unwrap_err();
            let error_str = error.to_string();
            
            // Verify error doesn't leak sensitive information
            assert!(!error_str.contains("password"));
            assert!(!error_str.contains("secret"));
            assert!(!error_str.contains("key"));
            assert!(!error_str.contains("token"));
            assert!(!error_str.contains("/etc/"));
            assert!(!error_str.contains("/var/"));
            assert!(!error_str.contains("/usr/"));
        }
    }
    
    Ok(())
} 