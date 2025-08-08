use rhema_git::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Git Conflict Resolution Example");
    println!("=======================================");

    // Create a temporary directory for our test repository
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    println!("\n📁 Creating test repository at: {}", repo_path.display());

    // Initialize a new Git repository
    let repo = git2::Repository::init(repo_path)?;

    // Create initial files
    let readme_path = repo_path.join("README.md");
    fs::write(
        &readme_path,
        "# Test Project\n\nThis is a test project for conflict resolution.",
    )?;

    // Create initial commit
    let signature = git2::Signature::now("Rhema Git", "rhema@example.com")?;
    let mut index = repo.index()?;
    index.add_path(Path::new("README.md"))?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    repo.commit(
        Some("refs/heads/main"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;

    // Create develop branch
    let main_commit = repo.find_commit(repo.head()?.target().unwrap())?;
    repo.branch("develop", &main_commit, false)?;

    println!("✅ Repository initialized with main and develop branches");

    // Create advanced git integration
    let mut git_integration = create_advanced_git_integration(repo_path)?;

    println!("\n🔍 Conflict Detection Demo:");
    println!("===========================");

    // Check for conflicts (should be none initially)
    let conflicts = git_integration.detect_conflicts()?;
    println!("  📊 Initial conflicts detected: {}", conflicts.len());

    println!("\n📋 Feature Branch with Potential Conflicts:");
    println!("===========================================");

    // Create a feature branch
    let feature_branch = git_integration.create_feature_branch("conflict-test", "develop")?;
    println!("  ✅ Created feature branch: {}", feature_branch.name);

    // Add a file that will be modified in both branches
    let shared_file = repo_path.join("shared.rs");
    fs::write(
        &shared_file,
        "pub fn shared_function() {\n    println!(\"Feature branch version\");\n}",
    )?;

    // Commit the changes
    let mut index = repo.index()?;
    index.add_path(Path::new("shared.rs"))?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let current_commit = repo.find_commit(repo.head()?.target().unwrap())?;
    repo.commit(
        Some("refs/heads/feature/conflict-test"),
        &signature,
        &signature,
        "Add shared function in feature branch",
        &tree,
        &[&current_commit],
    )?;

    println!("  📝 Added shared.rs with feature branch version");

    // Checkout develop and modify the same file
    let develop_branch = repo.find_branch("develop", git2::BranchType::Local)?;
    let develop_commit = develop_branch.get().peel_to_commit()?;

    let mut checkout_options = git2::build::CheckoutBuilder::new();
    checkout_options.force();
    repo.checkout_tree(&develop_commit.as_object(), Some(&mut checkout_options))?;
    repo.set_head("refs/heads/develop")?;

    // Modify the same file in develop
    fs::write(
        &shared_file,
        "pub fn shared_function() {\n    println!(\"Develop branch version\");\n}",
    )?;

    // Commit the changes
    let mut index = repo.index()?;
    index.add_path(Path::new("shared.rs"))?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let current_commit = repo.find_commit(repo.head()?.target().unwrap())?;
    repo.commit(
        Some("refs/heads/develop"),
        &signature,
        &signature,
        "Modify shared function in develop",
        &tree,
        &[&current_commit],
    )?;

    println!("  📝 Modified shared.rs in develop branch");

    println!("\n🔧 Conflict Resolution Strategies:");
    println!("==================================");

    // Demonstrate different conflict resolution strategies
    let strategies = vec![
        ("Current Version", AutoResolutionStrategy::Current),
        ("Incoming Version", AutoResolutionStrategy::Incoming),
        ("Base Version", AutoResolutionStrategy::Base),
        ("Merge Both", AutoResolutionStrategy::Merge),
    ];

    for (name, strategy) in strategies {
        println!("\n  🔄 Testing strategy: {}", name);

        // Create a test branch for this strategy
        let test_branch_name = format!("test-{}", name.to_lowercase().replace(" ", "-"));
        let test_branch = git_integration.create_feature_branch(&test_branch_name, "develop")?;

        // Try to finish the branch (this will create a conflict)
        let result = git_integration.finish_feature_branch(&test_branch_name);

        match result {
            Ok(result) => {
                if result.success {
                    println!("    ✅ Successfully resolved conflicts using {}", name);
                } else {
                    println!("    ❌ Failed to resolve conflicts using {}", name);
                    println!("    📝 Conflicts: {:?}", result.conflicts);
                }
            }
            Err(e) => {
                println!("    ❌ Error with {}: {}", name, e);
            }
        }
    }

    println!("\n📊 Conflict Resolution API:");
    println!("===========================");

    // Demonstrate the conflict resolution API
    let conflicts = git_integration.detect_conflicts()?;
    println!("  🔍 Detected conflicts: {}", conflicts.len());

    for conflict in &conflicts {
        println!("    📄 File: {}", conflict.file_path.display());
        println!("    🏷️  Type: {:?}", conflict.conflict_type);
        println!("    📝 Details: {}", conflict.details);
    }

    // Test manual resolution strategy
    println!("\n  🔧 Testing manual resolution strategy...");
    let manual_result = git_integration.resolve_conflicts(ConflictResolutionStrategy::Manual)?;
    println!("    📊 Manual resolution result: {}", manual_result.success);
    println!("    📝 Messages: {:?}", manual_result.messages);

    // Test abort strategy
    println!("\n  🛑 Testing abort strategy...");
    let abort_result = git_integration.resolve_conflicts(ConflictResolutionStrategy::Abort)?;
    println!("    📊 Abort result: {}", abort_result.success);
    println!("    📝 Messages: {:?}", abort_result.messages);

    println!("\n🎉 Conflict Resolution Example Completed!");
    println!("   The git crate now supports advanced conflict detection and resolution.");
    println!("   Multiple resolution strategies are available for different scenarios.");

    Ok(())
}
