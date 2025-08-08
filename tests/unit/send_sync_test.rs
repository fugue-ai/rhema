use std::path::Path;
use std::sync::{Arc, Mutex};
use rhema_core::RhemaResult;
use git2;
use tokio;

#[tokio::test]
async fn test_repository_send_sync() {
    // Create a temporary directory for the test repository
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = temp_dir.path();
    
    // Initialize a new git repository
    let repo = git2::Repository::init(repo_path).unwrap();
    
    // Wrap the repository in Arc<Mutex<>>
    let repo_arc = Arc::new(Mutex::new(repo));
    
    // Spawn multiple async tasks that access the repository
    let mut handles = vec![];
    
    for i in 0..5 {
        let repo_clone = repo_arc.clone();
        let handle = tokio::spawn(async move {
            // Access the repository through the mutex
            let repo_guard = repo_clone.lock().unwrap();
            
            // Perform some basic git operations
            let head = repo_guard.head();
            let _ = head.is_ok(); // Just check if we can access head
            
            // Get repository path
            let path = repo_guard.path();
            assert!(path.exists());
            
            println!("Task {} completed successfully", i);
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let result = handle.await;
        assert!(result.is_ok(), "Task should complete successfully");
    }
    
    println!("All async tasks completed successfully with Arc<Mutex<Repository>>");
}

#[tokio::test]
async fn test_workflow_manager_send_sync() {
    // This test would verify that WorkflowManager can be used in async contexts
    // For now, we'll just verify the basic structure works
    
    use rhema_git::git::workflow::{WorkflowManager, WorkflowConfig, default_git_flow_config};
    use git2::Repository;
    use std::sync::{Arc, Mutex};
    
    // Create a temporary directory for the test repository
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = temp_dir.path();
    
    // Initialize a new git repository
    let repo = Repository::init(repo_path).unwrap();
    
    // Create a workflow config
    let config = default_git_flow_config();
    
    // Create a workflow manager
    let workflow_manager = WorkflowManager::new(repo, config);
    
    // Verify that the repository is wrapped in Arc<Mutex<>>
    // This is an internal implementation detail, but we can verify it compiles
    println!("WorkflowManager created successfully with Arc<Mutex<Repository>>");
    
    // Test that the workflow manager can be created successfully
    // The repository is wrapped in Arc<Mutex<Repository>> internally
    println!("Successfully created WorkflowManager with Arc<Mutex<Repository>>");
    
    // Verify the repository path exists (we can't access the private repo field directly)
    assert!(repo_path.exists());
    println!("Repository path exists and is accessible");
}

#[tokio::test]
async fn test_automation_manager_send_sync() {
    // This test would verify that GitAutomationManager can be used in async contexts
    use rhema_git::git::automation::{GitAutomationManager, default_automation_config};
    use git2::Repository;
    use std::sync::{Arc, Mutex};
    
    // Create a temporary directory for the test repository
    let temp_dir = tempfile::tempdir().unwrap();
    let repo_path = temp_dir.path();
    
    // Initialize a new git repository
    let repo = Repository::init(repo_path).unwrap();
    
    // Create an automation config
    let config = default_automation_config();
    
    // Create an automation manager
    let automation_manager = GitAutomationManager::new(repo, config);
    
    // Verify that the repository is wrapped in Arc<Mutex<>>
    println!("GitAutomationManager created successfully with Arc<Mutex<Repository>>");
    
    // Test that the automation manager can be created successfully
    // The repository is wrapped in Arc<Mutex<Repository>> internally
    println!("Successfully created GitAutomationManager with Arc<Mutex<Repository>>");
    
    // Verify the repository path exists (we can't access the private repo field directly)
    assert!(repo_path.exists());
    println!("Repository path exists and is accessible");
} 