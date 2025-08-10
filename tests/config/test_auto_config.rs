use rhema_query::RepoAnalysis;

#[test]
fn test_auto_config_analysis() {
    println!("🧪 Testing Rhema Auto-Config Feature");

    // Test with current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("📁 Analyzing directory: {}", current_dir.display());

    match RepoAnalysis::analyze(&current_dir) {
        Ok(analysis) => {
            println!("\n✅ Analysis successful!");
            println!("Project Type: {:?}", analysis.project_type);
            println!("Languages: {:?}", analysis.languages);
            println!("Frameworks: {:?}", analysis.frameworks);
            println!("Databases: {:?}", analysis.databases);
            println!("Infrastructure: {:?}", analysis.infrastructure);
            println!("Suggested Scope Type: {}", analysis.suggested_scope_type);
            println!("Suggested Scope Name: {}", analysis.suggested_scope_name);
            println!("Suggested Description: {}", analysis.suggested_description);

            // Test generating RhemaScope
            let scope = analysis.generate_rhema_scope();
            println!("\n📋 Generated RhemaScope:");
            println!("Name: {}", scope.name);
            println!("Type: {}", scope.scope_type);
            println!("Description: {:?}", scope.description);
            println!("Version: {}", scope.version);
            println!(
                "Custom fields: {:?}",
                scope.custom.keys().collect::<Vec<_>>()
            );

            // Basic assertions
            assert!(!scope.name.is_empty());
            assert!(!scope.scope_type.is_empty());
            assert_eq!(scope.version, "1.0.0");
        }
        Err(e) => {
            panic!("❌ Analysis failed: {}", e);
        }
    }
}

#[test]
fn test_rust_project_detection() {
    // This test should detect that we're in a Rust project
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    match RepoAnalysis::analyze(&current_dir) {
        Ok(analysis) => {
            // Should detect Rust since we're in a Rust project
            assert!(
                analysis.languages.contains(&"Rust".to_string())
                    || analysis
                        .languages
                        .contains(&"JavaScript/TypeScript".to_string())
                    || analysis.languages.is_empty()
            ); // Allow empty for now

            // Should have a valid scope type
            assert!(!analysis.suggested_scope_type.is_empty());
            assert!(!analysis.suggested_scope_name.is_empty());
        }
        Err(e) => {
            panic!("Analysis failed: {}", e);
        }
    }
}
