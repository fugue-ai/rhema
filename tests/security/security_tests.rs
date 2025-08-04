use rhema_git::git::security::{
    SecurityManager, SecurityConfig, AccessControlConfig, AuditLoggingConfig,
    SecurityValidationConfig, EncryptionConfig, ThreatDetectionConfig,
    RolePermissions, Operation, LogLevel, AuditEvent, EncryptionAlgorithm,
    KeyStorage, KeyRotationPolicy, RetentionPolicy, SecurityScanningConfig,
    default_security_config
};
use git2::Repository;
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_security_manager_creation() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    
    // Initialize a test repository
    let repo = Repository::init(repo_path).unwrap();
    let config = default_security_config();
    
    let security_manager = SecurityManager::new(repo, config);
    assert!(security_manager.is_ok());
}

#[test]
fn test_access_validation() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    let repo = Repository::init(repo_path).unwrap();
    
    let mut config = default_security_config();
    config.enabled = true;
    
    // Add test role
    let mut roles = HashMap::new();
    let mut permissions = RolePermissions {
        name: "developer".to_string(),
        allowed_operations: vec![Operation::Read, Operation::Write, Operation::Commit],
        allowed_branches: vec!["feature/*".to_string()],
        allowed_files: vec!["src/**".to_string()],
        denied_files: vec!["config/secrets.*".to_string()],
    };
    roles.insert("test_user".to_string(), permissions);
    config.access_control.roles = roles;
    
    let security_manager = SecurityManager::new(repo, config).unwrap();
    
    // Test valid access
    let result = security_manager.validate_access("test_user", &Operation::Read, "src/main.rs");
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Test invalid operation
    let result = security_manager.validate_access("test_user", &Operation::Admin, "src/main.rs");
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_commit_security_validation() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    let repo = Repository::init(repo_path).unwrap();
    
    let mut config = default_security_config();
    config.enabled = true;
    config.validation.validate_signatures = true;
    config.validation.check_suspicious_patterns = true;
    config.validation.check_secrets = true;
    
    let security_manager = SecurityManager::new(repo, config).unwrap();
    
    // Create a test commit
    let signature = git2::Signature::now("Test User", "test@example.com").unwrap();
    let tree_id = repo.index().unwrap().write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    
    let commit_id = repo.commit(
        Some(&"refs/heads/main"),
        &signature,
        &signature,
        "Test commit message",
        &tree,
        &[],
    ).unwrap();
    
    let commit = repo.find_commit(commit_id).unwrap();
    let result = security_manager.validate_commit_security(&commit);
    
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    // Should have warnings about missing signature
    assert!(!validation_result.warnings.is_empty());
}

#[test]
fn test_secret_detection() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    let repo = Repository::init(repo_path).unwrap();
    
    let mut config = default_security_config();
    config.enabled = true;
    config.validation.check_secrets = true;
    
    let security_manager = SecurityManager::new(repo, config).unwrap();
    
    // Create a test commit with secrets
    let signature = git2::Signature::now("Test User", "test@example.com").unwrap();
    let mut index = repo.index().unwrap();
    
    // Create a file with secrets
    let secret_content = "password=secret123\napi_key=sk_test_1234567890abcdef";
    let secret_file_path = repo_path.join("config.txt");
    std::fs::write(&secret_file_path, secret_content).unwrap();
    
    index.add_path(&std::path::Path::new("config.txt")).unwrap();
    index.write().unwrap();
    
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    
    let commit_id = repo.commit(
        Some(&"refs/heads/main"),
        &signature,
        &signature,
        "Add config with secrets",
        &tree,
        &[],
    ).unwrap();
    
    let commit = repo.find_commit(commit_id).unwrap();
    let result = security_manager.validate_commit_security(&commit);
    
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    // Should detect secrets
    assert!(!validation_result.errors.is_empty());
}

#[test]
fn test_suspicious_pattern_detection() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    let repo = Repository::init(repo_path).unwrap();
    
    let mut config = default_security_config();
    config.enabled = true;
    config.validation.check_suspicious_patterns = true;
    
    let security_manager = SecurityManager::new(repo, config).unwrap();
    
    // Create a test commit with suspicious patterns
    let signature = git2::Signature::now("admin", "admin@test.com").unwrap();
    let mut index = repo.index().unwrap();
    
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    
    let commit_id = repo.commit(
        Some(&"refs/heads/main"),
        &signature,
        &signature,
        "URGENT: Fix critical security bug with password reset",
        &tree,
        &[],
    ).unwrap();
    
    let commit = repo.find_commit(commit_id).unwrap();
    let result = security_manager.validate_commit_security(&commit);
    
    assert!(result.is_ok());
    let validation_result = result.unwrap();
    // Should detect suspicious patterns
    assert!(!validation_result.warnings.is_empty());
}

#[test]
fn test_security_scanning() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    let repo = Repository::init(repo_path).unwrap();
    
    let mut config = default_security_config();
    config.enabled = true;
    config.validation.security_scanning.enabled = true;
    config.validation.security_scanning.scan_vulnerabilities = true;
    config.validation.security_scanning.scan_secrets = true;
    config.validation.security_scanning.scan_malware = true;
    
    let security_manager = SecurityManager::new(repo, config).unwrap();
    
    // Create test files with vulnerabilities
    let vulnerable_content = r#"
        $query = "SELECT * FROM users WHERE id = " . $_GET['id'];
        eval($code);
        console.log("debug info");
    "#;
    
    let vulnerable_file = repo_path.join("vulnerable.php");
    std::fs::write(&vulnerable_file, vulnerable_content).unwrap();
    
    let result = security_manager.run_security_scan(repo_path);
    assert!(result.is_ok());
    
    let scan_result = result.unwrap();
    // Should detect vulnerabilities
    assert!(!scan_result.vulnerabilities.is_empty());
}

#[test]
fn test_file_encryption() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    let repo = Repository::init(repo_path).unwrap();
    
    let mut config = default_security_config();
    config.enabled = true;
    config.encryption.enabled = true;
    config.encryption.algorithm = EncryptionAlgorithm::AES256;
    config.encryption.key_management.key_storage = KeyStorage::File(repo_path.join("key.txt"));
    
    let security_manager = SecurityManager::new(repo, config).unwrap();
    
    // Create a test file
    let test_content = "This is sensitive data that should be encrypted";
    let test_file = repo_path.join("sensitive.txt");
    std::fs::write(&test_file, test_content).unwrap();
    
    // Encrypt the file
    let encrypt_result = security_manager.encrypt_file(&test_file);
    assert!(encrypt_result.is_ok());
    
    // Check that encrypted file exists
    let encrypted_file = test_file.with_extension("encrypted");
    assert!(encrypted_file.exists());
    
    // Decrypt the file
    let decrypt_result = security_manager.decrypt_file(&encrypted_file);
    assert!(decrypt_result.is_ok());
    
    // Check that decrypted content matches original
    let decrypted_content = std::fs::read_to_string(&test_file).unwrap();
    assert_eq!(decrypted_content, test_content);
}

#[test]
fn test_audit_logging() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    let repo = Repository::init(repo_path).unwrap();
    
    let mut config = default_security_config();
    config.enabled = true;
    config.audit_logging.enabled = true;
    config.audit_logging.log_file = repo_path.join("audit.log");
    config.audit_logging.log_level = LogLevel::Info;
    config.audit_logging.events = vec![AuditEvent::Commit, AuditEvent::SecurityViolation];
    
    let security_manager = SecurityManager::new(repo, config).unwrap();
    
    // Perform an operation that should be logged
    let result = security_manager.validate_access("test_user", &Operation::Read, "test.txt");
    assert!(result.is_ok());
    
    // Check that log file was created
    assert!(config.audit_logging.log_file.exists());
}

#[test]
fn test_default_security_config() {
    let config = default_security_config();
    
    // Verify default values
    assert!(!config.enabled);
    assert!(config.access_control.require_authentication);
    assert!(config.access_control.rbac_enabled);
    assert!(config.audit_logging.enabled);
    assert!(config.validation.validate_signatures);
    assert!(config.validation.check_secrets);
    assert!(!config.encryption.enabled);
    assert!(!config.threat_detection.enabled);
}

#[test]
fn test_role_based_access_control() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();
    let repo = Repository::init(repo_path).unwrap();
    
    let mut config = default_security_config();
    config.enabled = true;
    
    // Create different roles
    let mut roles = HashMap::new();
    
    // Admin role
    let admin_permissions = RolePermissions {
        name: "admin".to_string(),
        allowed_operations: vec![
            Operation::Read, Operation::Write, Operation::Commit,
            Operation::Push, Operation::Pull, Operation::Merge,
            Operation::Rebase, Operation::Delete, Operation::Admin
        ],
        allowed_branches: vec!["*".to_string()],
        allowed_files: vec!["**".to_string()],
        denied_files: vec![],
    };
    roles.insert("admin".to_string(), admin_permissions);
    
    // Developer role
    let dev_permissions = RolePermissions {
        name: "developer".to_string(),
        allowed_operations: vec![
            Operation::Read, Operation::Write, Operation::Commit
        ],
        allowed_branches: vec!["feature/*".to_string(), "develop".to_string()],
        allowed_files: vec!["src/**".to_string()],
        denied_files: vec!["config/secrets.*".to_string()],
    };
    roles.insert("developer".to_string(), dev_permissions);
    
    // Read-only role
    let readonly_permissions = RolePermissions {
        name: "readonly".to_string(),
        allowed_operations: vec![Operation::Read],
        allowed_branches: vec!["main".to_string()],
        allowed_files: vec!["src/**".to_string()],
        denied_files: vec!["config/**".to_string()],
    };
    roles.insert("readonly".to_string(), readonly_permissions);
    
    config.access_control.roles = roles;
    
    let security_manager = SecurityManager::new(repo, config).unwrap();
    
    // Test admin access
    let result = security_manager.validate_access("admin", &Operation::Admin, "any/file.txt");
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Test developer access
    let result = security_manager.validate_access("developer", &Operation::Commit, "src/main.rs");
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Test developer denied access
    let result = security_manager.validate_access("developer", &Operation::Admin, "src/main.rs");
    assert!(result.is_ok());
    assert!(!result.unwrap());
    
    // Test readonly access
    let result = security_manager.validate_access("readonly", &Operation::Read, "src/main.rs");
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Test readonly denied write
    let result = security_manager.validate_access("readonly", &Operation::Write, "src/main.rs");
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_security_scan_result_management() {
    use rhema_git::git::security::{SecurityScanResult, SecurityIssue};
    
    let mut scan_result = SecurityScanResult::new();
    
    // Add various security findings
    scan_result.add_vulnerability("SQL injection in user input".to_string());
    scan_result.add_malware("Suspicious eval() usage".to_string());
    scan_result.add_secret("API key found in config".to_string());
    scan_result.add_info("Scan completed successfully".to_string());
    
    let issue = SecurityIssue {
        severity: "High".to_string(),
        category: "Vulnerability".to_string(),
        description: "Critical security issue".to_string(),
        file_path: Some(PathBuf::from("src/main.rs")),
        line_number: Some(42),
    };
    scan_result.add_issue(issue);
    
    // Verify results
    assert!(!scan_result.clean);
    assert_eq!(scan_result.vulnerabilities.len(), 1);
    assert_eq!(scan_result.malware.len(), 1);
    assert_eq!(scan_result.secrets.len(), 1);
    assert_eq!(scan_result.info.len(), 1);
    assert_eq!(scan_result.issues.len(), 1);
    
    // Test risk level setting
    scan_result.set_risk_level("High".to_string());
    assert_eq!(scan_result.risk_level, "High");
} 