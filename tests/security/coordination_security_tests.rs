//! Security tests for Rhema Coordination CLI

use crate::common::{fixtures::TestFixtures, helpers::TestHelpers, TestEnv};
use std::process::Command;
use tempfile::TempDir;

/// Test CLI command execution for coordination commands
fn run_coordination_command(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "rhema", "--", "coordination"])
        .args(args)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(format!(
            "Coordination command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}

// ============================================================================
// Input Validation Security Tests
// ============================================================================

#[test]
fn test_coordination_sql_injection_agent_name() {
    // Test SQL injection attempts in agent names
    let malicious_names = vec![
        "'; DROP TABLE agents; --",
        "' OR '1'='1",
        "'; INSERT INTO agents VALUES ('hacker', 'malicious'); --",
        "admin'--",
        "'; UPDATE agents SET type='hacker'; --",
    ];

    for malicious_name in malicious_names {
        let result = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            malicious_name,
            "--type",
            "TestAgent",
            "--scope",
            "security-testing",
        ]);

        // Should either fail gracefully or sanitize the input
        if result.is_ok() {
            let output = result.unwrap();
            // Verify the malicious content is not executed
            assert!(!output.contains("DROP TABLE"));
            assert!(!output.contains("INSERT INTO"));
            assert!(!output.contains("UPDATE"));
        }
    }
}

#[test]
fn test_coordination_xss_agent_name() {
    // Test XSS attempts in agent names
    let malicious_names = vec![
        "<script>alert('xss')</script>",
        "javascript:alert('xss')",
        "<img src=x onerror=alert('xss')>",
        "';alert('xss');//",
        "<svg onload=alert('xss')>",
    ];

    for malicious_name in malicious_names {
        let result = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            malicious_name,
            "--type",
            "TestAgent",
            "--scope",
            "security-testing",
        ]);

        // Should either fail gracefully or sanitize the input
        if result.is_ok() {
            let output = result.unwrap();
            // Verify the malicious content is not executed
            assert!(!output.contains("<script>"));
            assert!(!output.contains("javascript:"));
            assert!(!output.contains("onerror="));
            assert!(!output.contains("onload="));
        }
    }
}

#[test]
fn test_coordination_path_traversal_agent_name() {
    // Test path traversal attempts in agent names
    let malicious_names = vec![
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "/etc/passwd",
        "C:\\Windows\\System32\\config\\SAM",
        "....//....//....//etc/passwd",
    ];

    for malicious_name in malicious_names {
        let result = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            malicious_name,
            "--type",
            "TestAgent",
            "--scope",
            "security-testing",
        ]);

        // Should either fail gracefully or sanitize the input
        if result.is_ok() {
            let output = result.unwrap();
            // Verify no sensitive file access
            assert!(!output.contains("/etc/passwd"));
            assert!(!output.contains("system32"));
            assert!(!output.contains("SAM"));
        }
    }
}

#[test]
fn test_coordination_command_injection_agent_name() {
    // Test command injection attempts in agent names
    let malicious_names = vec![
        "$(rm -rf /)",
        "`rm -rf /`",
        "; rm -rf /;",
        "| rm -rf /",
        "&& rm -rf /",
        "$(cat /etc/passwd)",
        "`cat /etc/passwd`",
    ];

    for malicious_name in malicious_names {
        let result = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            malicious_name,
            "--type",
            "TestAgent",
            "--scope",
            "security-testing",
        ]);

        // Should either fail gracefully or sanitize the input
        if result.is_ok() {
            let output = result.unwrap();
            // Verify no command execution
            assert!(!output.contains("rm -rf"));
            assert!(!output.contains("/etc/passwd"));
        }
    }
}

#[test]
fn test_coordination_buffer_overflow_agent_name() {
    // Test buffer overflow attempts with extremely long names
    let long_names = vec!["A".repeat(10000), "A".repeat(100000), "A".repeat(1000000)];

    for long_name in long_names {
        let result = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            &long_name,
            "--type",
            "TestAgent",
            "--scope",
            "security-testing",
        ]);

        // Should handle gracefully without crashing
        // Either succeed with truncation or fail with proper error
        if result.is_err() {
            let error = result.unwrap_err();
            // Should not be a panic or crash
            assert!(!error.to_string().contains("panicked"));
            assert!(!error.to_string().contains("overflow"));
        }
    }
}

#[test]
fn test_coordination_null_byte_injection() {
    // Test null byte injection attempts
    let malicious_names = vec![
        "agent\x00name",
        "agent\x00",
        "\x00agent",
        "agent\x00\x00name",
    ];

    for malicious_name in malicious_names {
        let result = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            malicious_name,
            "--type",
            "TestAgent",
            "--scope",
            "security-testing",
        ]);

        // Should handle null bytes gracefully
        if result.is_err() {
            let error = result.unwrap_err();
            // Should not crash or panic
            assert!(!error.to_string().contains("panicked"));
        }
    }
}

// ============================================================================
// Message Payload Security Tests
// ============================================================================

#[test]
fn test_coordination_malicious_message_payload() {
    let env = TestEnv::with_sample_data().unwrap();
    std::env::set_current_dir(&env.repo_path).unwrap();

    // Register an agent first
    run_coordination_command(&[
        "agent",
        "register",
        "--name",
        "security-test-agent",
        "--type",
        "TestAgent",
        "--scope",
        "security-testing",
    ])
    .unwrap();

    // Test malicious JSON payloads
    let malicious_payloads = vec![
        r#"{"script": "<script>alert('xss')</script>"}"#,
        r#"{"command": "$(rm -rf /)"}"#,
        r#"{"path": "../../../etc/passwd"}"#,
        r#"{"sql": "'; DROP TABLE agents; --"}"#,
        r#"{"eval": "eval('alert(1)')"}"#,
        r#"{"exec": "exec('rm -rf /')"}"#,
    ];

    for malicious_payload in malicious_payloads {
        let result = run_coordination_command(&[
            "agent",
            "send-message",
            "--to",
            "agent-001",
            "Test message",
            "--message-type",
            "Test",
            "--priority",
            "Normal",
            "--payload",
            malicious_payload,
        ]);

        // Should either fail gracefully or sanitize the payload
        if result.is_ok() {
            let output = result.unwrap();
            // Verify the malicious content is not executed
            assert!(!output.contains("<script>"));
            assert!(!output.contains("rm -rf"));
            assert!(!output.contains("/etc/passwd"));
            assert!(!output.contains("DROP TABLE"));
            assert!(!output.contains("eval("));
            assert!(!output.contains("exec("));
        }
    }
}

#[test]
fn test_coordination_invalid_json_payload() {
    let env = TestEnv::with_sample_data().unwrap();
    std::env::set_current_dir(&env.repo_path).unwrap();

    // Register an agent first
    run_coordination_command(&[
        "agent",
        "register",
        "--name",
        "json-test-agent",
        "--type",
        "TestAgent",
        "--scope",
        "security-testing",
    ])
    .unwrap();

    // Test invalid JSON payloads
    let invalid_payloads = vec![
        "{invalid json}",
        "{'key': 'value'}",     // Single quotes instead of double
        "{key: value}",         // Missing quotes
        "{",                    // Incomplete JSON
        "}",                    // Incomplete JSON
        "[1, 2, 3,",            // Incomplete array
        r#"{"key": "value",}"#, // Trailing comma
    ];

    for invalid_payload in invalid_payloads {
        let result = run_coordination_command(&[
            "agent",
            "send-message",
            "--to",
            "agent-001",
            "Test message",
            "--message-type",
            "Test",
            "--priority",
            "Normal",
            "--payload",
            invalid_payload,
        ]);

        // Should fail gracefully with proper error message
        assert!(result.is_err());
        let error = result.unwrap_err();
        // Should not crash or panic
        assert!(!error.to_string().contains("panicked"));
    }
}

#[test]
fn test_coordination_oversized_payload() {
    let env = TestEnv::with_sample_data().unwrap();
    std::env::set_current_dir(&env.repo_path).unwrap();

    // Register an agent first
    run_coordination_command(&[
        "agent",
        "register",
        "--name",
        "size-test-agent",
        "--type",
        "TestAgent",
        "--scope",
        "security-testing",
    ])
    .unwrap();

    // Create oversized payloads
    let oversized_payloads = vec![
        format!(r#"{{"data": "{}"}}"#, "A".repeat(1000000)), // 1MB payload
        format!(r#"{{"data": "{}"}}"#, "B".repeat(10000000)), // 10MB payload
    ];

    for oversized_payload in oversized_payloads {
        let result = run_coordination_command(&[
            "agent",
            "send-message",
            "--to",
            "agent-001",
            "Test message",
            "--message-type",
            "Test",
            "--priority",
            "Normal",
            "--payload",
            &oversized_payload,
        ]);

        // Should handle gracefully without crashing
        if result.is_err() {
            let error = result.unwrap_err();
            // Should not be a panic or crash
            assert!(!error.to_string().contains("panicked"));
            assert!(!error.to_string().contains("overflow"));
        }
    }
}

// ============================================================================
// Authentication and Authorization Tests
// ============================================================================

#[test]
fn test_coordination_unauthorized_agent_access() {
    // Test accessing agent information without proper authentication
    let result = run_coordination_command(&["agent", "info", "non-existent-agent-id"]);

    // Should fail with proper error message
    assert!(result.is_err());
    let error = result.unwrap_err();
    // Should not expose sensitive information
    assert!(!error.to_string().contains("password"));
    assert!(!error.to_string().contains("secret"));
    assert!(!error.to_string().contains("token"));
}

#[test]
fn test_coordination_unauthorized_session_access() {
    // Test accessing session information without proper authentication
    let result = run_coordination_command(&["session", "info", "non-existent-session-id"]);

    // Should fail with proper error message
    assert!(result.is_err());
    let error = result.unwrap_err();
    // Should not expose sensitive information
    assert!(!error.to_string().contains("password"));
    assert!(!error.to_string().contains("secret"));
    assert!(!error.to_string().contains("token"));
}

#[test]
fn test_coordination_privilege_escalation_attempt() {
    // Test attempts to escalate privileges through agent registration
    let malicious_agent_types = vec![
        "AdminAgent",
        "RootAgent",
        "SuperUserAgent",
        "SystemAgent",
        "PrivilegedAgent",
    ];

    for agent_type in malicious_agent_types {
        let result = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            "privilege-test-agent",
            "--type",
            agent_type,
            "--scope",
            "security-testing",
        ]);

        // Should either fail or not grant elevated privileges
        if result.is_ok() {
            let output = result.unwrap();
            // Verify no privilege escalation
            assert!(!output.contains("admin"));
            assert!(!output.contains("root"));
            assert!(!output.contains("privileged"));
        }
    }
}

// ============================================================================
// Session Security Tests
// ============================================================================

#[test]
fn test_coordination_session_hijacking_attempt() {
    let env = TestEnv::with_sample_data().unwrap();
    std::env::set_current_dir(&env.repo_path).unwrap();

    // Create a session
    run_coordination_command(&[
        "session",
        "create",
        "Security Test Session",
        "--participants",
        "agent-001",
    ])
    .unwrap();

    // Test session hijacking attempts
    let malicious_session_ids = vec![
        "session-001", // Try to access existing session
        "session-002", // Try to access non-existent session
        "admin-session",
        "root-session",
    ];

    for session_id in malicious_session_ids {
        let result = run_coordination_command(&[
            "session",
            "send-message",
            "--session-id",
            session_id,
            "Hijacking attempt",
            "--message-type",
            "Test",
            "--priority",
            "Normal",
            "--sender-id",
            "unauthorized-agent",
        ]);

        // Should fail for unauthorized access
        if result.is_ok() {
            let output = result.unwrap();
            // Verify no unauthorized access
            assert!(!output.contains("hijacked"));
            assert!(!output.contains("unauthorized"));
        }
    }
}

#[test]
fn test_coordination_session_injection() {
    let env = TestEnv::with_sample_data().unwrap();
    std::env::set_current_dir(&env.repo_path).unwrap();

    // Test session creation with malicious input
    let malicious_session_names = vec![
        "Session'; DROP TABLE sessions; --",
        "<script>alert('session xss')</script>",
        "$(rm -rf /)",
        "../../../etc/passwd",
    ];

    for malicious_name in malicious_session_names {
        let result = run_coordination_command(&[
            "session",
            "create",
            malicious_name,
            "--participants",
            "agent-001",
        ]);

        // Should either fail gracefully or sanitize the input
        if result.is_ok() {
            let output = result.unwrap();
            // Verify the malicious content is not executed
            assert!(!output.contains("DROP TABLE"));
            assert!(!output.contains("<script>"));
            assert!(!output.contains("rm -rf"));
            assert!(!output.contains("/etc/passwd"));
        }
    }
}

// ============================================================================
// System Monitoring Security Tests
// ============================================================================

#[test]
fn test_coordination_system_stats_information_disclosure() {
    // Test that system stats don't expose sensitive information
    let result = run_coordination_command(&["system", "stats", "--detailed"]);

    if result.is_ok() {
        let output = result.unwrap();
        // Should not expose sensitive information
        assert!(!output.contains("password"));
        assert!(!output.contains("secret"));
        assert!(!output.contains("token"));
        assert!(!output.contains("key"));
        assert!(!output.contains("credential"));
    }
}

#[test]
fn test_coordination_message_history_information_disclosure() {
    // Test that message history doesn't expose sensitive information
    let result = run_coordination_command(&["system", "message-history", "--show-payloads"]);

    if result.is_ok() {
        let output = result.unwrap();
        // Should not expose sensitive information
        assert!(!output.contains("password"));
        assert!(!output.contains("secret"));
        assert!(!output.contains("token"));
        assert!(!output.contains("key"));
        assert!(!output.contains("credential"));
    }
}

// ============================================================================
// Denial of Service Tests
// ============================================================================

#[test]
fn test_coordination_dos_rapid_registration() {
    // Test rapid agent registration to cause DoS
    let start_time = std::time::Instant::now();
    let mut success_count = 0;
    let mut error_count = 0;

    // Try to register many agents rapidly
    for i in 0..1000 {
        match run_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("dos-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "dos-testing",
        ]) {
            Ok(_) => success_count += 1,
            Err(_) => error_count += 1,
        }

        // Limit test duration
        if start_time.elapsed().as_secs() > 30 {
            break;
        }
    }

    let duration = start_time.elapsed();

    println!("=== DoS Test Results ===");
    println!("Duration: {:?}", duration);
    println!("Successful: {}", success_count);
    println!("Errors: {}", error_count);
    println!(
        "Operations per second: {:.2}",
        (success_count + error_count) as f64 / duration.as_secs_f64()
    );
    println!("========================");

    // Should handle gracefully without crashing
    assert!(duration.as_secs() < 60); // Should not hang indefinitely
}

#[test]
fn test_coordination_dos_rapid_messaging() {
    let env = TestEnv::with_sample_data().unwrap();
    std::env::set_current_dir(&env.repo_path).unwrap();

    // Register an agent first
    run_coordination_command(&[
        "agent",
        "register",
        "--name",
        "dos-message-agent",
        "--type",
        "TestAgent",
        "--scope",
        "dos-testing",
    ])
    .unwrap();

    let start_time = std::time::Instant::now();
    let mut success_count = 0;
    let mut error_count = 0;

    // Try to send many messages rapidly
    for i in 0..1000 {
        match run_coordination_command(&[
            "agent",
            "send-message",
            "--to",
            "agent-001",
            &format!("DoS test message {}", i),
            "--message-type",
            "Test",
            "--priority",
            "Normal",
        ]) {
            Ok(_) => success_count += 1,
            Err(_) => error_count += 1,
        }

        // Limit test duration
        if start_time.elapsed().as_secs() > 30 {
            break;
        }
    }

    let duration = start_time.elapsed();

    println!("=== DoS Messaging Test Results ===");
    println!("Duration: {:?}", duration);
    println!("Successful: {}", success_count);
    println!("Errors: {}", error_count);
    println!(
        "Messages per second: {:.2}",
        (success_count + error_count) as f64 / duration.as_secs_f64()
    );
    println!("================================");

    // Should handle gracefully without crashing
    assert!(duration.as_secs() < 60); // Should not hang indefinitely
}

// ============================================================================
// Resource Exhaustion Tests
// ============================================================================

#[test]
fn test_coordination_memory_exhaustion() {
    // Test memory exhaustion through large payloads
    let large_payload = format!(r#"{{"data": "{}"}}"#, "X".repeat(10000000)); // 10MB

    let result = run_coordination_command(&[
        "agent",
        "register",
        "--name",
        "memory-test-agent",
        "--type",
        "TestAgent",
        "--scope",
        "memory-testing",
        "--capabilities",
        &large_payload,
    ]);

    // Should handle gracefully without crashing
    if result.is_err() {
        let error = result.unwrap_err();
        // Should not be a panic or crash
        assert!(!error.to_string().contains("panicked"));
        assert!(!error.to_string().contains("out of memory"));
    }
}

#[test]
fn test_coordination_cpu_exhaustion() {
    // Test CPU exhaustion through complex operations
    let start_time = std::time::Instant::now();

    // Perform many complex operations
    for i in 0..100 {
        let _ = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("cpu-test-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "cpu-testing",
            "--capabilities",
            &format!("capability-{}", i),
        ]);

        let _ = run_coordination_command(&["agent", "list", "--detailed"]);
        let _ = run_coordination_command(&["system", "stats", "--detailed"]);
    }

    let duration = start_time.elapsed();

    // Should complete within reasonable time
    assert!(duration.as_secs() < 120); // Should not hang indefinitely
}

// ============================================================================
// Input Sanitization Tests
// ============================================================================

#[test]
fn test_coordination_input_sanitization() {
    // Test various input sanitization scenarios
    let test_cases = vec![
        ("agent name", "normal-agent-name"),
        ("agent name with spaces", "agent name with spaces"),
        ("agent-name-with-dashes", "agent-name-with-dashes"),
        ("agent_name_with_underscores", "agent_name_with_underscores"),
        ("agent123", "agent123"),
        ("AGENT", "AGENT"),
        ("agent", "agent"),
    ];

    for (description, agent_name) in test_cases {
        let result = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            agent_name,
            "--type",
            "TestAgent",
            "--scope",
            "sanitization-testing",
        ]);

        // Should handle all valid inputs
        if result.is_err() {
            let error = result.unwrap_err();
            // Should not be a security-related failure
            assert!(!error.to_string().contains("security"));
            assert!(!error.to_string().contains("malicious"));
        }
    }
}

#[test]
fn test_coordination_unicode_handling() {
    // Test Unicode character handling
    let unicode_names = vec![
        "agent-测试",
        "agent-тест",
        "agent-اختبار",
        "agent-🎉",
        "agent-🚀",
        "agent-🌟",
    ];

    for unicode_name in unicode_names {
        let result = run_coordination_command(&[
            "agent",
            "register",
            "--name",
            unicode_name,
            "--type",
            "TestAgent",
            "--scope",
            "unicode-testing",
        ]);

        // Should handle Unicode gracefully
        if result.is_err() {
            let error = result.unwrap_err();
            // Should not be a security-related failure
            assert!(!error.to_string().contains("security"));
            assert!(!error.to_string().contains("malicious"));
        }
    }
}
