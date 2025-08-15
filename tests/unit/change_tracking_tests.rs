use chrono::{Duration, Utc};
use rhema_core::{
    schema::*,
    scope::Scope,
    RhemaError, RhemaResult,
};
use rhema_mcp::context::{
    ChangeType, ContextChange, ContextProvider, ResourceType,
};
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_get_changes_since() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_root = temp_dir.path().to_path_buf();
    
    // Create a context provider
    let context = ContextProvider::new(repo_root)?;
    
    // Get changes since 1 hour ago
    let one_hour_ago = Utc::now() - Duration::hours(1);
    let changes = context.get_changes_since(one_hour_ago).await?;
    
    // Initially there should be no changes
    assert_eq!(changes.len(), 0);
    
    Ok(())
}

#[tokio::test]
async fn test_get_recent_changes() -> RhemaResult<()> {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new()?;
    let repo_root = temp_dir.path().to_path_buf();
    
    // Create a context provider
    let context = ContextProvider::new(repo_root)?;
    
    // Test getting changes for different time periods
    let changes_hour = context.get_changes_last_hour().await?;
    let changes_day = context.get_changes_last_day().await?;
    let changes_week = context.get_changes_last_week().await?;
    
    // Initially there should be no changes
    assert_eq!(changes_hour.len(), 0);
    assert_eq!(changes_day.len(), 0);
    assert_eq!(changes_week.len(), 0);
    
    Ok(())
}

#[tokio::test]
async fn test_change_types() -> RhemaResult<()> {
    // Test that ChangeType enum works correctly
    let created = ChangeType::Created;
    let updated = ChangeType::Updated;
    let deleted = ChangeType::Deleted;
    
    // Test serialization/deserialization
    let created_json = serde_json::to_string(&created)?;
    let updated_json = serde_json::to_string(&updated)?;
    let deleted_json = serde_json::to_string(&deleted)?;
    
    assert_eq!(created_json, "\"Created\"");
    assert_eq!(updated_json, "\"Updated\"");
    assert_eq!(deleted_json, "\"Deleted\"");
    
    Ok(())
}

#[tokio::test]
async fn test_resource_types() -> RhemaResult<()> {
    // Test that ResourceType enum works correctly
    let scope = ResourceType::Scope;
    let knowledge = ResourceType::Knowledge;
    let todo = ResourceType::Todo;
    let decision = ResourceType::Decision;
    let pattern = ResourceType::Pattern;
    let convention = ResourceType::Convention;
    let lock_file = ResourceType::LockFile;
    
    // Test serialization/deserialization
    let scope_json = serde_json::to_string(&scope)?;
    let knowledge_json = serde_json::to_string(&knowledge)?;
    let todo_json = serde_json::to_string(&todo)?;
    let decision_json = serde_json::to_string(&decision)?;
    let pattern_json = serde_json::to_string(&pattern)?;
    let convention_json = serde_json::to_string(&convention)?;
    let lock_file_json = serde_json::to_string(&lock_file)?;
    
    assert_eq!(scope_json, "\"Scope\"");
    assert_eq!(knowledge_json, "\"Knowledge\"");
    assert_eq!(todo_json, "\"Todo\"");
    assert_eq!(decision_json, "\"Decision\"");
    assert_eq!(pattern_json, "\"Pattern\"");
    assert_eq!(convention_json, "\"Convention\"");
    assert_eq!(lock_file_json, "\"LockFile\"");
    
    Ok(())
}

#[tokio::test]
async fn test_context_change_serialization() -> RhemaResult<()> {
    // Test that ContextChange can be serialized and deserialized
    let change = ContextChange {
        scope_path: "test/scope".to_string(),
        change_type: ChangeType::Created,
        resource_type: ResourceType::Knowledge,
        resource_id: Some("test-id".to_string()),
        timestamp: Utc::now(),
        details: Some(serde_json::json!({
            "title": "Test Knowledge",
            "content_length": 100
        })),
    };
    
    // Serialize
    let json = serde_json::to_string(&change)?;
    
    // Deserialize
    let deserialized: ContextChange = serde_json::from_str(&json)?;
    
    // Verify fields
    assert_eq!(deserialized.scope_path, "test/scope");
    assert_eq!(deserialized.change_type, ChangeType::Created);
    assert_eq!(deserialized.resource_type, ResourceType::Knowledge);
    assert_eq!(deserialized.resource_id, Some("test-id".to_string()));
    
    Ok(())
}
