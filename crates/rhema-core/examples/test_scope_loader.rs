use rhema_core::scope_loader::service::ScopeLoaderConfig;
use rhema_core::scope_loader::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Rhema Scope Loader Plugin System Test");
    println!("==========================================");

    // Create a plugin registry
    let mut registry = PluginRegistry::new();

    // Register our built-in plugins
    println!("📦 Registering plugins...");
    registry.register_plugin(Box::new(CargoPlugin::new()))?;
    registry.register_plugin(Box::new(NodePackagePlugin::new()))?;
    registry.register_plugin(Box::new(NxPlugin::new()))?;

    println!("✅ Registered {} plugins", registry.plugin_count());

    // List all registered plugins
    println!("\n📋 Registered Plugins:");
    for plugin_info in registry.list_plugins() {
        println!(
            "  • {} (v{}) - {}",
            plugin_info.metadata.name,
            plugin_info.metadata.version,
            plugin_info.metadata.description
        );
        println!(
            "    Supported package managers: {}",
            plugin_info.metadata.supported_package_managers.join(", ")
        );
        println!("    Priority: {}", plugin_info.metadata.priority);
    }

    // Test with current directory
    let current_dir = std::env::current_dir()?;
    println!("\n🔍 Testing scope discovery in: {}", current_dir.display());

    // Detect package boundaries
    println!("\n📦 Detecting package boundaries...");
    match registry.detect_boundaries(&current_dir) {
        Ok(boundaries) => {
            println!("✅ Detected {} package boundaries:", boundaries.len());
            for boundary in &boundaries {
                println!(
                    "  📦 {} ({}) at {}",
                    boundary.package_info.name,
                    boundary.package_manager.as_str(),
                    boundary.path.display()
                );

                if !boundary.dependencies.is_empty() {
                    println!(
                        "    Dependencies: {}",
                        boundary
                            .dependencies
                            .iter()
                            .map(|d| format!("{}@{}", d.name, d.version))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }

                if !boundary.scripts.is_empty() {
                    println!(
                        "    Scripts: {}",
                        boundary
                            .scripts
                            .keys()
                            .cloned()
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to detect boundaries: {}", e);
        }
    }

    // Generate scope suggestions
    println!("\n💡 Generating scope suggestions...");
    match registry.suggest_scopes(&current_dir) {
        Ok(suggestions) => {
            println!("✅ Generated {} scope suggestions:", suggestions.len());
            for suggestion in &suggestions {
                println!(
                    "  🎯 {} ({}) - Confidence: {:.2}",
                    suggestion.name,
                    suggestion.scope_type.as_str(),
                    suggestion.confidence
                );
                println!("    Path: {}", suggestion.path.display());
                println!("    Reasoning: {}", suggestion.reasoning);

                if !suggestion.dependencies.is_empty() {
                    println!("    Dependencies: {}", suggestion.dependencies.join(", "));
                }

                if !suggestion.files.is_empty() {
                    println!("    Files: {} files", suggestion.files.len());
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to generate suggestions: {}", e);
        }
    }

    // Create scope loader service
    println!("\n🔧 Creating scope loader service...");
    let config = ScopeLoaderConfig {
        min_confidence_threshold: 0.8,
        auto_create: false, // Don't auto-create for this test
        confirm_prompt: false,
        max_depth: 5,
        enable_caching: true,
        cache_duration: 3600,
        cache_path: None,
    };

    let service = ScopeLoaderService::new(registry, config);

    // Test service methods
    println!("\n🔧 Testing Scope Loader Service:");

    // Detect boundaries using service
    println!("📦 Detecting boundaries with service...");
    match service.detect_boundaries(&current_dir).await {
        Ok(boundaries) => {
            println!("✅ Service detected {} boundaries", boundaries.len());
        }
        Err(e) => {
            println!("❌ Service failed to detect boundaries: {}", e);
        }
    }

    // Generate suggestions using service
    println!("💡 Generating suggestions with service...");
    match service.suggest_scopes(&current_dir).await {
        Ok(suggestions) => {
            println!("✅ Service generated {} suggestions", suggestions.len());
        }
        Err(e) => {
            println!("❌ Service failed to generate suggestions: {}", e);
        }
    }

    // Get cache stats
    let cache_stats = service.cache_stats().await;
    println!(
        "📊 Cache stats: {} boundaries, {} suggestions, {} scopes",
        cache_stats.boundaries_count, cache_stats.suggestions_count, cache_stats.scopes_count
    );

    // Test with a specific path if provided
    if let Some(test_path) = std::env::args().nth(1) {
        let test_path = PathBuf::from(test_path);
        if test_path.exists() {
            println!("\n🔍 Testing with specific path: {}", test_path.display());

            match service.detect_boundaries(&test_path).await {
                Ok(boundaries) => {
                    println!(
                        "✅ Detected {} boundaries in {}",
                        boundaries.len(),
                        test_path.display()
                    );
                }
                Err(e) => {
                    println!(
                        "❌ Failed to detect boundaries in {}: {}",
                        test_path.display(),
                        e
                    );
                }
            }
        } else {
            println!("❌ Test path does not exist: {}", test_path.display());
        }
    }

    println!("\n🎉 Scope Loader Plugin System test completed!");
    println!("💡 Try running with a specific path: cargo run --bin test_scope_loader <path>");

    Ok(())
}
