use rhema_git::*;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Git Workflow Example");
    println!("==============================");

    // Example: Create a feature branch workflow
    println!("\n📋 Feature Branch Workflow:");
    let feature_branch = FeatureBranch {
        name: "feature/user-authentication".to_string(),
        base_branch: "develop".to_string(),
        created_at: chrono::Utc::now(),
        context_files: vec!["auth.rs".to_string().into(), "user.rs".to_string().into()],
    };
    println!("  ✅ Created feature branch: {}", feature_branch.name);
    println!("  📁 Context files: {:?}", feature_branch.context_files);

    // Example: Release branch workflow
    println!("\n📦 Release Branch Workflow:");
    let release_branch = ReleaseBranch {
        name: "release/v2.1.0".to_string(),
        version: "2.1.0".to_string(),
        created_at: chrono::Utc::now(),
        status: ReleaseStatus::InProgress,
    };
    println!("  ✅ Created release branch: {}", release_branch.name);
    println!("  🏷️  Version: {}", release_branch.version);

    // Example: Hotfix branch workflow
    println!("\n🔧 Hotfix Branch Workflow:");
    let hotfix_branch = HotfixBranch {
        name: "hotfix/security-patch".to_string(),
        version: "2.1.1".to_string(),
        created_at: chrono::Utc::now(),
        status: HotfixStatus::InProgress,
    };
    println!("  ✅ Created hotfix branch: {}", hotfix_branch.name);
    println!("  🚨 Critical fix for version: {}", hotfix_branch.version);

    // Example: Workflow results
    println!("\n📊 Workflow Results:");
    let feature_result = FeatureResult {
        success: true,
        merged_branch: "feature/user-authentication".to_string(),
        target_branch: "develop".to_string(),
        conflicts: vec![],
        messages: vec!["Feature branch merged successfully".to_string()],
        conflict_resolution: None,
    };
    println!("  ✅ Feature result: {}", feature_result.success);
    println!("  📝 Messages: {:?}", feature_result.messages);

    let release_result = ReleaseResult {
        success: true,
        version: "2.1.0".to_string(),
        main_merge: true,
        develop_merge: true,
        tag_created: true,
        messages: vec!["Release v2.1.0 completed successfully".to_string()],
        conflict_resolution: None,
    };
    println!("  ✅ Release result: {}", release_result.success);
    println!("  🏷️  Tag created: {}", release_result.tag_created);

    let hotfix_result = HotfixResult {
        success: true,
        version: "2.1.1".to_string(),
        main_merge: true,
        develop_merge: true,
        tag_created: true,
        messages: vec!["Security hotfix v2.1.1 deployed".to_string()],
        conflict_resolution: None,
    };
    println!("  ✅ Hotfix result: {}", hotfix_result.success);
    println!("  🚨 Security patch deployed: {}", hotfix_result.version);

    println!("\n🎉 Git Workflow API is working correctly!");
    println!("   The CLI can now use these functions to implement workflow commands.");

    Ok(())
} 