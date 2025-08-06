use rhema_git::*;
use std::path::Path;
use tempfile::TempDir;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Git Real Workflow Example");
    println!("===================================");

    // Create a temporary directory for our test repository
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();
    
    println!("\n📁 Creating test repository at: {}", repo_path.display());
    
    // Initialize a new Git repository
    let repo = git2::Repository::init(repo_path)?;
    
    // Create initial files
    let readme_path = repo_path.join("README.md");
    fs::write(&readme_path, "# Test Project\n\nThis is a test project for Rhema Git workflows.")?;
    
    // Create initial commit
    let signature = git2::Signature::now("Rhema Git", "rhema@example.com")?;
    let mut index = repo.index()?;
    index.add_path(Path::new("README.md"))?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    repo.commit(Some("refs/heads/main"), &signature, &signature, "Initial commit", &tree, &[])?;
    
    // Create develop branch
    let main_commit = repo.find_commit(repo.head()?.target().unwrap())?;
    repo.branch("develop", &main_commit, false)?;
    
    println!("✅ Repository initialized with main and develop branches");

    // Create advanced git integration
    let mut git_integration = create_advanced_git_integration(repo_path)?;
    
    println!("\n📋 Feature Branch Workflow:");
    println!("============================");
    
    // Create a feature branch
    let feature_branch = git_integration.create_feature_branch("user-authentication", "develop")?;
    println!("  ✅ Created feature branch: {}", feature_branch.name);
    println!("  📅 Created at: {}", feature_branch.created_at);
    
    // Add some files to the feature branch
    let auth_file = repo_path.join("auth.rs");
    fs::write(&auth_file, "pub struct Auth { /* authentication logic */ }")?;
    
    let user_file = repo_path.join("user.rs");
    fs::write(&user_file, "pub struct User { /* user management */ }")?;
    
    // Stage and commit the changes
    let mut index = repo.index()?;
    index.add_path(Path::new("auth.rs"))?;
    index.add_path(Path::new("user.rs"))?;
    index.write()?;
    
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let current_commit = repo.find_commit(repo.head()?.target().unwrap())?;
    repo.commit(Some("refs/heads/feature/user-authentication"), &signature, &signature, 
        "Add user authentication feature", &tree, &[&current_commit])?;
    
    println!("  📝 Added authentication files and committed changes");
    
    // Checkout develop branch for merging
    let develop_branch = repo.find_branch("develop", git2::BranchType::Local)?;
    let develop_commit = develop_branch.get().peel_to_commit()?;
    
    // Use checkout options to force checkout
    let mut checkout_options = git2::build::CheckoutBuilder::new();
    checkout_options.force();
    repo.checkout_tree(&develop_commit.as_object(), Some(&mut checkout_options))?;
    repo.set_head("refs/heads/develop")?;
    
    // Finish the feature branch
    let feature_result = git_integration.finish_feature_branch("user-authentication")?;
    println!("  ✅ Feature branch finished: {}", feature_result.success);
    println!("  📝 Messages: {:?}", feature_result.messages);
    
    println!("\n📦 Release Branch Workflow:");
    println!("============================");
    
    // Create a release branch
    let release_branch = git_integration.start_release_branch("1.0.0")?;
    println!("  ✅ Created release branch: {}", release_branch.name);
    println!("  🏷️  Version: {}", release_branch.version);
    
    // Add release notes
    let release_notes = repo_path.join("RELEASE_NOTES.md");
    fs::write(&release_notes, "# Release 1.0.0\n\n- Added user authentication\n- Improved performance\n- Bug fixes")?;
    
    // Commit release notes
    let mut index = repo.index()?;
    index.add_path(Path::new("RELEASE_NOTES.md"))?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let current_commit = repo.find_commit(repo.head()?.target().unwrap())?;
    repo.commit(Some("refs/heads/release/1.0.0"), &signature, &signature, 
        "Add release notes for v1.0.0", &tree, &[&current_commit])?;
    
    println!("  📝 Added release notes and committed changes");
    
    // Checkout main for release
    let main_branch = repo.find_branch("main", git2::BranchType::Local)?;
    let main_commit = main_branch.get().peel_to_commit()?;
    let mut checkout_options = git2::build::CheckoutBuilder::new();
    checkout_options.force();
    repo.checkout_tree(&main_commit.as_object(), Some(&mut checkout_options))?;
    repo.set_head("refs/heads/main")?;
    
    // Finish the release branch
    let release_result = git_integration.finish_release_branch("1.0.0")?;
    println!("  ✅ Release branch finished: {}", release_result.success);
    println!("  🏷️  Tag created: {}", release_result.tag_created);
    println!("  📝 Messages: {:?}", release_result.messages);
    
    println!("\n🔧 Hotfix Branch Workflow:");
    println!("===========================");
    
    // Create a hotfix branch
    let hotfix_branch = git_integration.start_hotfix_branch("1.0.1")?;
    println!("  ✅ Created hotfix branch: {}", hotfix_branch.name);
    println!("  🚨 Critical fix for version: {}", hotfix_branch.version);
    
    // Add security fix
    let security_fix = repo_path.join("security.rs");
    fs::write(&security_fix, "pub fn security_patch() { /* critical security fix */ }")?;
    
    // Commit the security fix
    let mut index = repo.index()?;
    index.add_path(Path::new("security.rs"))?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let current_commit = repo.find_commit(repo.head()?.target().unwrap())?;
    repo.commit(Some("refs/heads/hotfix/1.0.1"), &signature, &signature, 
        "Critical security patch", &tree, &[&current_commit])?;
    
    println!("  🔒 Added security patch and committed changes");
    
    // Checkout main for hotfix
    let main_branch = repo.find_branch("main", git2::BranchType::Local)?;
    let main_commit = main_branch.get().peel_to_commit()?;
    let mut checkout_options = git2::build::CheckoutBuilder::new();
    checkout_options.force();
    repo.checkout_tree(&main_commit.as_object(), Some(&mut checkout_options))?;
    repo.set_head("refs/heads/main")?;
    
    // Finish the hotfix branch
    let hotfix_result = git_integration.finish_hotfix_branch("1.0.1")?;
    println!("  ✅ Hotfix branch finished: {}", hotfix_result.success);
    println!("  🚨 Security patch deployed: {}", hotfix_result.version);
    println!("  📝 Messages: {:?}", hotfix_result.messages);
    
    println!("\n📊 Workflow Status:");
    println!("===================");
    
    // Get current workflow status
    let workflow_status = git_integration.get_workflow_status()?;
    println!("  🌿 Current branch: {}", workflow_status.current_branch);
    println!("  📋 Branch type: {:?}", workflow_status.branch_type);
    println!("  🔄 Workflow type: {:?}", workflow_status.workflow_type);
    println!("  📈 Status: {}", workflow_status.status);
    
    println!("\n🎉 Real Git Workflow Operations Completed Successfully!");
    println!("   All operations used actual Git commands and repository management.");
    println!("   The git crate now supports real workflow automation!");
    
    Ok(())
} 