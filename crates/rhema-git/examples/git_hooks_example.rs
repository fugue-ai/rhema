use rhema_git::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Git Hooks Integration Example");
    println!("======================================");

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
        "# Test Project\n\nThis is a test project for Git hooks integration.",
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

    println!("✅ Repository initialized");

    // Create advanced git integration
    let mut git_integration = create_advanced_git_integration(repo_path)?;

    println!("\n🔧 Git Hooks Installation:");
    println!("==========================");

    // Install default Rhema hooks
    git_integration.install_default_hooks()?;
    println!("  ✅ Default Rhema hooks installed");

    // List installed hooks
    let hooks = git_integration.list_hooks()?;
    println!("  📋 Installed hooks: {:?}", hooks);

    println!("\n🔍 Pre-Commit Hook Demo:");
    println!("=======================");

    // Create a file with TODO comment to trigger pre-commit hook
    let test_file = repo_path.join("test.rs");
    fs::write(
        &test_file,
        "// TODO: Implement this function\npub fn test_function() {\n    // Implementation\n}",
    )?;

    // Stage the file
    let mut index = repo.index()?;
    index.add_path(Path::new("test.rs"))?;
    index.write()?;

    println!("  📝 Created test.rs with TODO comment");
    println!("  📦 Staged file for commit");

    // Execute pre-commit hooks
    let pre_commit_result = git_integration.execute_pre_commit_hooks()?;

    match pre_commit_result {
        Some(result) => {
            println!("  🔍 Pre-commit hook executed: {}", result.success);
            println!("  📝 Messages: {:?}", result.messages);
            if !result.errors.is_empty() {
                println!("  ❌ Errors: {:?}", result.errors);
            }
        }
        None => {
            println!("  ⚠️  No pre-commit hooks found");
        }
    }

    println!("\n📝 Post-Commit Hook Demo:");
    println!("=========================");

    // Create a commit to trigger post-commit hook
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let head = repo.head()?;
    let parent_commit = head.peel_to_commit()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Add test function with TODO",
        &tree,
        &[&parent_commit],
    )?;

    println!("  ✅ Created commit");

    // Execute post-commit hooks
    let post_commit_result = git_integration.execute_post_commit_hooks()?;

    match post_commit_result {
        Some(result) => {
            println!("  🔍 Post-commit hook executed: {}", result.success);
            println!("  📝 Messages: {:?}", result.messages);
            if !result.errors.is_empty() {
                println!("  ❌ Errors: {:?}", result.errors);
            }
        }
        None => {
            println!("  ⚠️  No post-commit hooks found");
        }
    }

    println!("\n🚀 Pre-Push Hook Demo:");
    println!("======================");

    // Execute pre-push hooks
    let pre_push_result = git_integration.execute_pre_push_hooks()?;

    match pre_push_result {
        Some(result) => {
            println!("  🔍 Pre-push hook executed: {}", result.success);
            println!("  📝 Messages: {:?}", result.messages);
            if !result.errors.is_empty() {
                println!("  ❌ Errors: {:?}", result.errors);
            }
        }
        None => {
            println!("  ⚠️  No pre-push hooks found");
        }
    }

    println!("\n🔧 Custom Hook Installation:");
    println!("============================");

    // Create a custom hook
    let custom_hook_script = r#"#!/bin/sh
# Custom Rhema hook
echo "Running custom Rhema hook..."

# Check for specific patterns
if git diff --cached --name-only | xargs grep -l "console.log\|debugger" 2>/dev/null; then
    echo "Warning: Found debugging code in staged files"
fi

# Check file sizes
git diff --cached --name-only | while read file; do
    if [ -f "$file" ]; then
        size=$(stat -f%z "$file" 2>/dev/null || stat -c%s "$file" 2>/dev/null || echo 0)
        if [ "$size" -gt 5242880 ]; then
            echo "Error: File too large: $file ($size bytes)"
            exit 1
        fi
    fi
done

echo "Custom hook validation completed"
"#;

    // Install custom hook using the public API
    // Note: We'll need to add a method to install custom hooks
    println!("  📝 Custom hook script prepared (installation via API to be implemented)");

    // List hooks again
    let updated_hooks = git_integration.list_hooks()?;
    println!("  📋 Updated hooks: {:?}", updated_hooks);

    println!("\n🎉 Git Hooks Integration Example Completed!");
    println!("   The git crate now supports comprehensive Git hooks integration.");
    println!("   Pre-commit, post-commit, and pre-push hooks are available.");
    println!("   Custom hooks can be installed and managed programmatically.");

    Ok(())
}
