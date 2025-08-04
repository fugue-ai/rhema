/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use git2::Repository;
use rhema_git::git::workflow::{WorkflowManager, default_git_flow_config};
use rhema_git::git::version_management::{
    VersionManagementConfig, BumpType, VersioningStrategy, ChangelogConfig,
    ReleaseNotesConfig, CommitPatterns, AutoBumpConfig, default_version_management_config
};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Git Version Management Example");
    println!("=====================================\n");

    // Initialize repository
    let repo_path = std::env::current_dir()?;
    let repo = Repository::open(&repo_path)?;
    
    // Create workflow configuration
    let workflow_config = default_git_flow_config();
    
    // Create custom version management configuration
    let version_config = create_custom_version_config();
    
    // Create workflow manager with version management
    let workflow_manager = WorkflowManager::new(repo, workflow_config)
        .with_version_manager(version_config);

    // Example 1: Get current version
    println!("📋 Example 1: Get Current Version");
    println!("--------------------------------");
    match workflow_manager.get_current_version() {
        Ok(version) => println!("Current version: {}", version),
        Err(e) => println!("Error getting version: {}", e),
    }
    println!();

    // Example 2: Validate version
    println!("✅ Example 2: Validate Version");
    println!("-----------------------------");
    let test_version = "1.2.3";
    match workflow_manager.validate_version(test_version) {
        Ok(errors) => {
            if errors.is_empty() {
                println!("Version {} is valid", test_version);
            } else {
                println!("Version {} has validation errors:", test_version);
                for error in errors {
                    println!("  - {}", error);
                }
            }
        }
        Err(e) => println!("Error validating version: {}", e),
    }
    println!();

    // Example 3: Bump version (simulated)
    println!("🔄 Example 3: Bump Version");
    println!("-------------------------");
    println!("Note: This is a simulation - no actual files will be modified");
    
    // Simulate different bump types
    let bump_types = vec![
        ("Patch", BumpType::Patch),
        ("Minor", BumpType::Minor),
        ("Major", BumpType::Major),
    ];

    for (name, bump_type) in bump_types {
        println!("\nSimulating {} version bump:", name);
        match workflow_manager.bump_version(Some(bump_type)).await {
            Ok(result) => {
                println!("  Success: {}", result.success);
                if let Some(old_version) = &result.old_version {
                    println!("  Old version: {}", old_version);
                }
                if let Some(new_version) = &result.new_version {
                    println!("  New version: {}", new_version);
                }
                println!("  Changelog generated: {}", result.changelog_generated);
                println!("  Release notes generated: {}", result.release_notes_generated);
                for message in &result.messages {
                    println!("  Message: {}", message);
                }
            }
            Err(e) => println!("  Error: {}", e),
        }
    }
    println!();

    // Example 4: Generate changelog
    println!("📝 Example 4: Generate Changelog");
    println!("-------------------------------");
    let version = "2.0.0";
    match workflow_manager.generate_changelog(version).await {
        Ok(()) => println!("Changelog generated successfully for version {}", version),
        Err(e) => println!("Error generating changelog: {}", e),
    }
    println!();

    // Example 5: Generate release notes
    println!("📄 Example 5: Generate Release Notes");
    println!("-----------------------------------");
    match workflow_manager.generate_release_notes(version).await {
        Ok(()) => println!("Release notes generated successfully for version {}", version),
        Err(e) => println!("Error generating release notes: {}", e),
    }
    println!();

    // Example 6: Complete release workflow
    println!("🎯 Example 6: Complete Release Workflow");
    println!("--------------------------------------");
    demonstrate_release_workflow(&workflow_manager).await?;

    println!("✅ Version Management Example Completed!");
    Ok(())
}

/// Create a custom version management configuration
fn create_custom_version_config() -> VersionManagementConfig {
    let mut config = default_version_management_config();
    
    // Customize versioning strategy
    config.strategy = VersioningStrategy::Semantic;
    
    // Customize changelog configuration
    config.changelog = ChangelogConfig {
        file_path: PathBuf::from("CHANGELOG.md"),
        format: rhema_git::git::version_management::ChangelogFormat::Markdown,
        include_commit_hashes: true,
        include_author: true,
        include_date: true,
        group_by_type: true,
        commit_types: std::collections::HashMap::new(),
        template: None,
    };
    
    // Customize release notes configuration
    config.release_notes = ReleaseNotesConfig {
        directory: PathBuf::from("release-notes"),
        format: rhema_git::git::version_management::ReleaseNotesFormat::Markdown,
        include_breaking_changes: true,
        include_migration_guide: true,
        include_security_notes: true,
        template: None,
    };
    
    // Customize commit patterns
    config.commit_patterns = CommitPatterns {
        major_bump: vec![
            "!breaking".to_string(),
            "BREAKING CHANGE".to_string(),
            "major".to_string(),
        ],
        minor_bump: vec![
            "feat".to_string(),
            "feature".to_string(),
            "minor".to_string(),
        ],
        patch_bump: vec![
            "fix".to_string(),
            "bugfix".to_string(),
            "patch".to_string(),
        ],
        breaking_change: vec![
            "!breaking".to_string(),
            "BREAKING CHANGE".to_string(),
        ],
        ignore: vec![
            "docs".to_string(),
            "style".to_string(),
            "chore".to_string(),
        ],
    };
    
    // Customize auto-bump configuration
    config.auto_bump = AutoBumpConfig {
        enabled: true,
        strategy: rhema_git::git::version_management::BumpStrategy::Conservative,
        analyze_commits: true,
        analyze_changes: false,
        min_confidence: 0.8,
        confirm_major_bumps: true,
    };
    
    config
}

/// Demonstrate a complete release workflow
async fn demonstrate_release_workflow(workflow_manager: &WorkflowManager) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting release workflow simulation...");
    
    // Step 1: Get current version
    let current_version = workflow_manager.get_current_version()?;
    println!("  Current version: {}", current_version);
    
    // Step 2: Validate current version
    let validation_errors = workflow_manager.validate_version(&current_version)?;
    if !validation_errors.is_empty() {
        println!("  Validation errors found:");
        for error in validation_errors {
            println!("    - {}", error);
        }
        return Ok(());
    }
    println!("  Version validation passed");
    
    // Step 3: Determine next version (simulate minor bump)
    println!("  Determining next version...");
    match workflow_manager.bump_version(Some(BumpType::Minor)).await {
        Ok(result) => {
            if let Some(new_version) = &result.new_version {
                println!("  Next version: {}", new_version);
                
                // Step 4: Generate changelog
                println!("  Generating changelog...");
                if let Err(e) = workflow_manager.generate_changelog(new_version).await {
                    println!("    Error generating changelog: {}", e);
                } else {
                    println!("    Changelog generated successfully");
                }
                
                // Step 5: Generate release notes
                println!("  Generating release notes...");
                if let Err(e) = workflow_manager.generate_release_notes(new_version).await {
                    println!("    Error generating release notes: {}", e);
                } else {
                    println!("    Release notes generated successfully");
                }
                
                println!("  Release workflow completed successfully!");
            }
        }
        Err(e) => println!("  Error during version bump: {}", e),
    }
    
    Ok(())
}

/// Example of using version management with different strategies
#[allow(dead_code)]
async fn demonstrate_version_strategies() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 Version Strategy Examples");
    println!("===========================");
    
    let repo = Repository::open(std::env::current_dir()?)?;
    let workflow_config = default_git_flow_config();
    
    // Semantic versioning strategy
    let mut semantic_config = default_version_management_config();
    semantic_config.strategy = VersioningStrategy::Semantic;
    
    let semantic_manager = WorkflowManager::new(repo.clone(), workflow_config.clone())
        .with_version_manager(semantic_config);
    
    println!("Semantic Versioning:");
    match semantic_manager.get_current_version() {
        Ok(version) => println!("  Current: {}", version),
        Err(e) => println!("  Error: {}", e),
    }
    
    // Calendar versioning strategy
    let mut calendar_config = default_version_management_config();
    calendar_config.strategy = VersioningStrategy::Calendar;
    
    let calendar_manager = WorkflowManager::new(repo.clone(), workflow_config.clone())
        .with_version_manager(calendar_config);
    
    println!("Calendar Versioning:");
    match calendar_manager.get_current_version() {
        Ok(version) => println!("  Current: {}", version),
        Err(e) => println!("  Error: {}", e),
    }
    
    // Custom versioning strategy
    let mut custom_config = default_version_management_config();
    custom_config.strategy = VersioningStrategy::Custom("YYYY.MM.DD-HH".to_string());
    
    let custom_manager = WorkflowManager::new(repo, workflow_config)
        .with_version_manager(custom_config);
    
    println!("Custom Versioning:");
    match custom_manager.get_current_version() {
        Ok(version) => println!("  Current: {}", version),
        Err(e) => println!("  Error: {}", e),
    }
    
    Ok(())
}

/// Example of commit message analysis for version bumping
#[allow(dead_code)]
fn demonstrate_commit_analysis() {
    println!("📊 Commit Message Analysis Examples");
    println!("==================================");
    
    let commit_messages = vec![
        "feat: add new user authentication system",
        "fix: resolve memory leak in cache manager",
        "docs: update API documentation",
        "feat!: breaking change in user API",
        "style: format code according to style guide",
        "refactor: improve database connection handling",
        "test: add unit tests for user service",
        "chore: update dependencies",
    ];
    
    for message in commit_messages {
        let bump_type = analyze_commit_message(&message);
        println!("  {} -> {:?}", message, bump_type);
    }
}

/// Analyze commit message to determine bump type
fn analyze_commit_message(message: &str) -> BumpType {
    let message_lower = message.to_lowercase();
    
    if message_lower.contains("!breaking") || message_lower.contains("breaking change") {
        BumpType::Major
    } else if message_lower.starts_with("feat") || message_lower.contains("feature") {
        BumpType::Minor
    } else if message_lower.starts_with("fix") || message_lower.contains("bugfix") {
        BumpType::Patch
    } else {
        BumpType::None
    }
} 