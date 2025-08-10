use std::path::Path;

use crate::scope_loader::*;

/// Example function demonstrating how to use the scope loader plugin system
pub async fn test_scope_loader() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Testing Scope Loader Plugin System");
    
    // Create a plugin registry
    let mut registry = PluginRegistry::new();
    
    // Register our built-in plugins
    registry.register_plugin(Box::new(CargoPlugin::new()))?;
    registry.register_plugin(Box::new(NodePackagePlugin::new()))?;
    registry.register_plugin(Box::new(NxPlugin::new()))?;
    
    println!("✅ Registered {} plugins", registry.plugin_count());
    
    // List all registered plugins
    for plugin_info in registry.list_plugins() {
        println!("📦 Plugin: {} (v{}) - {}", 
                 plugin_info.metadata.name, 
                 plugin_info.metadata.version, 
                 plugin_info.metadata.description);
    }
    
    // Test with current directory
    let current_dir = std::env::current_dir()?;
    println!("\n🔍 Testing scope discovery in: {}", current_dir.display());
    
    // Detect package boundaries
    match registry.detect_boundaries(&current_dir) {
        Ok(boundaries) => {
            println!("✅ Detected {} package boundaries:", boundaries.len());
            for boundary in &boundaries {
                println!("  📦 {} ({}) at {}", 
                         boundary.package_info.name,
                         boundary.package_manager.as_str(),
                         boundary.path.display());
            }
        }
        Err(e) => {
            println!("❌ Failed to detect boundaries: {}", e);
        }
    }
    
    // Generate scope suggestions
    match registry.suggest_scopes(&current_dir) {
        Ok(suggestions) => {
            println!("\n💡 Generated {} scope suggestions:", suggestions.len());
            for suggestion in &suggestions {
                println!("  🎯 {} ({}) - Confidence: {:.2} - {}", 
                         suggestion.name,
                         suggestion.scope_type.as_str(),
                         suggestion.confidence,
                         suggestion.reasoning);
            }
        }
        Err(e) => {
            println!("❌ Failed to generate suggestions: {}", e);
        }
    }
    
    // Create scope loader service
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
    match service.detect_boundaries(&current_dir).await {
        Ok(boundaries) => {
            println!("✅ Service detected {} boundaries", boundaries.len());
        }
        Err(e) => {
            println!("❌ Service failed to detect boundaries: {}", e);
        }
    }
    
    // Generate suggestions using service
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
    println!("📊 Cache stats: {} boundaries, {} suggestions, {} scopes", 
             cache_stats.boundaries_count, 
             cache_stats.suggestions_count, 
             cache_stats.scopes_count);
    
    println!("\n🎉 Scope Loader Plugin System test completed!");
    Ok(())
}

/// Example function showing how to create a custom plugin
pub fn create_custom_plugin_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Creating custom plugin example...");
    
    // This would be a custom plugin implementation
    // For now, we'll just show the structure
    
    struct CustomPlugin;
    
    impl ScopeLoaderPlugin for CustomPlugin {
        fn metadata(&self) -> PluginMetadata {
            PluginMetadata {
                name: "custom".to_string(),
                version: "1.0.0".to_string(),
                description: "Custom plugin example".to_string(),
                supported_package_managers: vec!["custom".to_string()],
                priority: 50,
            }
        }
        
        fn can_handle(&self, _path: &Path) -> bool {
            // Custom logic to determine if this plugin can handle the path
            false
        }
        
        fn detect_boundaries(&self, _path: &Path) -> Result<Vec<PackageBoundary>, PluginError> {
            // Custom logic to detect package boundaries
            Ok(Vec::new())
        }
        
        fn suggest_scopes(&self, _boundaries: &[PackageBoundary]) -> Result<Vec<ScopeSuggestion>, PluginError> {
            // Custom logic to generate scope suggestions
            Ok(Vec::new())
        }
        
        fn create_scopes(&self, _suggestions: &[ScopeSuggestion]) -> Result<Vec<crate::scope::Scope>, PluginError> {
            // Custom logic to create scopes
            Ok(Vec::new())
        }
        
        fn load_context(&self, _scope: &crate::scope::Scope) -> Result<ScopeContext, PluginError> {
            // Custom logic to load scope context
            Ok(ScopeContext {
                scope_name: "custom".to_string(),
                package_manager: PackageManager::Custom("custom".to_string()),
                dependencies: Vec::new(),
                scripts: std::collections::HashMap::new(),
                metadata: std::collections::HashMap::new(),
            })
        }
    }
    
    println!("✅ Custom plugin example created!");
    Ok(())
}
