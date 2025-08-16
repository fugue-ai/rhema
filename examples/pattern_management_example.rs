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

use rhema_core::{
    fileops::pattern_management::*,
    schema::knowledge::{PatternMaturity, PatternUsage, ReviewStatus},
    RhemaResult,
};
use std::path::Path;

/// Comprehensive example demonstrating advanced pattern management features
#[tokio::main]
async fn main() -> RhemaResult<()> {
    println!("🚀 Pattern Management Advanced Features Example");
    println!("{}", "=".repeat(60));

    // Create a temporary directory for testing
    let temp_dir = std::env::temp_dir().join("rhema_pattern_example");
    std::fs::create_dir_all(&temp_dir)?;

    // Step 1: Create sample patterns
    println!("\n📝 Step 1: Creating sample patterns...");
    create_sample_patterns(&temp_dir).await?;

    // Step 2: Export patterns
    println!("\n📤 Step 2: Exporting patterns...");
    export_patterns_example(&temp_dir).await?;

    // Step 3: Validate patterns
    println!("\n🔍 Step 3: Validating patterns...");
    validate_patterns_example(&temp_dir).await?;

    // Step 4: Generate documentation
    println!("\n📚 Step 4: Generating documentation...");
    generate_documentation_example(&temp_dir).await?;

    // Step 5: Import patterns
    println!("\n📥 Step 5: Importing patterns...");
    import_patterns_example(&temp_dir).await?;

    // Step 6: Analyze patterns
    println!("\n📊 Step 6: Analyzing patterns...");
    analyze_patterns_example(&temp_dir).await?;

    println!("\n✅ Pattern management example completed successfully!");
    println!("📁 Check the temporary directory: {}", temp_dir.display());

    Ok(())
}

async fn create_sample_patterns(scope_path: &Path) -> RhemaResult<()> {
    // Create a sample pattern for API design
    let api_pattern_id = add_enhanced_pattern_entry(
        scope_path,
        "RESTful API Design".to_string(),
        "Design patterns for creating RESTful APIs with proper resource modeling and HTTP semantics".to_string(),
        "API Design".to_string(),
        PatternUsage::Recommended,
        Some(9),
        Some("Use nouns for resources, proper HTTP status codes, versioning strategy".to_string()),
        Some("RPC-style endpoints, inconsistent naming, mixed HTTP methods".to_string()),
        Some("API Design".to_string()),
        Some(PatternMaturity::Mature),
        Some(7),
        Some(5),
        Some(8),
        Some("Authentication, authorization, input validation".to_string()),
        Some("Unit tests, integration tests, API documentation".to_string()),
        Some("OpenAPI/Swagger documentation, API versioning guide".to_string()),
        Some("HTTP client library, JSON serializer".to_string()),
        Some("Web services, microservices, public APIs".to_string()),
        Some("1.0.0".to_string()),
        Some("API Team".to_string()),
        Some(ReviewStatus::Approved),
    )?;

    // Create a sample pattern for error handling
    let error_pattern_id = add_enhanced_pattern_entry(
        scope_path,
        "Structured Error Handling".to_string(),
        "Consistent error handling patterns with proper error codes and user-friendly messages"
            .to_string(),
        "Error Handling".to_string(),
        PatternUsage::Required,
        Some(8),
        Some("Use error codes, provide context, log errors appropriately".to_string()),
        Some("Generic error messages, silent failures, inconsistent error formats".to_string()),
        Some("Error Handling".to_string()),
        Some(PatternMaturity::Mature),
        Some(6),
        Some(4),
        Some(7),
        Some("Error information disclosure, logging sensitive data".to_string()),
        Some("Error scenario tests, logging verification".to_string()),
        Some("Error handling guide, logging standards".to_string()),
        Some("Logging framework, error tracking service".to_string()),
        Some("All applications, especially user-facing ones".to_string()),
        Some("1.1.0".to_string()),
        Some("Platform Team".to_string()),
        Some(ReviewStatus::Approved),
    )?;

    // Create a sample pattern for caching
    let cache_pattern_id = add_enhanced_pattern_entry(
        scope_path,
        "Multi-Level Caching Strategy".to_string(),
        "Implement caching at multiple levels for optimal performance".to_string(),
        "Performance".to_string(),
        PatternUsage::Recommended,
        Some(7),
        Some("Browser cache, CDN, application cache, database cache".to_string()),
        Some("Cache everything, ignore cache invalidation, no TTL".to_string()),
        Some("Performance".to_string()),
        Some(PatternMaturity::Experimental),
        Some(8),
        Some(6),
        Some(9),
        Some("Cache poisoning, sensitive data exposure".to_string()),
        Some("Cache hit/miss tests, performance benchmarks".to_string()),
        Some("Caching strategy document, TTL guidelines".to_string()),
        Some("Redis, Memcached, CDN service".to_string()),
        Some("High-traffic applications, data-heavy operations".to_string()),
        Some("0.9.0".to_string()),
        Some("Performance Team".to_string()),
        Some(ReviewStatus::InReview),
    )?;

    println!("   ✅ Created 3 sample patterns:");
    println!("      • {} (API Design)", api_pattern_id);
    println!("      • {} (Error Handling)", error_pattern_id);
    println!("      • {} (Caching)", cache_pattern_id);

    Ok(())
}

async fn export_patterns_example(scope_path: &Path) -> RhemaResult<()> {
    // Export to JSON
    let json_export = export_patterns(
        scope_path,
        ExportFormat::Json,
        None,  // all pattern types
        None,  // all categories
        false, // no templates
        true,  // include stats
    )?;

    let json_file = scope_path.join("patterns_export.json");
    std::fs::write(&json_file, json_export)?;
    println!("   ✅ Exported to JSON: {}", json_file.display());

    // Export to YAML
    let yaml_export = export_patterns(
        scope_path,
        ExportFormat::Yaml,
        Some("API Design".to_string()), // only API patterns
        None,
        false,
        false,
    )?;

    let yaml_file = scope_path.join("api_patterns.yaml");
    std::fs::write(&yaml_file, yaml_export)?;
    println!(
        "   ✅ Exported API patterns to YAML: {}",
        yaml_file.display()
    );

    // Export to CSV
    let csv_export = export_patterns(
        scope_path,
        ExportFormat::Csv,
        None,
        Some("Performance".to_string()), // only performance patterns
        false,
        false,
    )?;

    let csv_file = scope_path.join("performance_patterns.csv");
    std::fs::write(&csv_file, csv_export)?;
    println!(
        "   ✅ Exported performance patterns to CSV: {}",
        csv_file.display()
    );

    Ok(())
}

async fn validate_patterns_example(scope_path: &Path) -> RhemaResult<()> {
    // Basic validation
    let basic_validation = validate_patterns(
        scope_path,
        None, // validate all patterns
        ValidationLevel::Basic,
        false, // no auto-fix
    )?;

    println!("   📊 Basic validation results:");
    println!(
        "      • Total patterns: {}",
        basic_validation.total_patterns
    );
    println!("      • Valid: {}", basic_validation.valid_count);
    println!("      • Invalid: {}", basic_validation.invalid_count);
    println!("      • Warnings: {}", basic_validation.warnings_count);

    // Strict validation
    let strict_validation = validate_patterns(scope_path, None, ValidationLevel::Strict, false)?;

    println!("   📊 Strict validation results:");
    println!(
        "      • Total patterns: {}",
        strict_validation.total_patterns
    );
    println!("      • Valid: {}", strict_validation.valid_count);
    println!("      • Invalid: {}", strict_validation.invalid_count);
    println!("      • Warnings: {}", strict_validation.warnings_count);

    if !strict_validation.warnings.is_empty() {
        println!("   ⚠️  Warnings found:");
        for warning in &strict_validation.warnings[..2] {
            // Show first 2 warnings
            println!("      • {}", warning);
        }
    }

    // Comprehensive validation
    let comprehensive_validation =
        validate_patterns(scope_path, None, ValidationLevel::Comprehensive, false)?;

    println!("   📊 Comprehensive validation results:");
    println!(
        "      • Total patterns: {}",
        comprehensive_validation.total_patterns
    );
    println!("      • Valid: {}", comprehensive_validation.valid_count);
    println!(
        "      • Invalid: {}",
        comprehensive_validation.invalid_count
    );
    println!(
        "      • Warnings: {}",
        comprehensive_validation.warnings_count
    );

    Ok(())
}

async fn generate_documentation_example(scope_path: &Path) -> RhemaResult<()> {
    let docs_dir = scope_path.join("pattern_docs");

    // Generate markdown documentation
    let docs_result = generate_pattern_documentation(
        scope_path,
        Some(&docs_dir),
        DocFormat::Markdown,
        true,  // include examples
        false, // no templates
        true,  // include stats
        true,  // generate index
    )?;

    println!("   📚 Documentation generated:");
    println!("      • Output directory: {}", docs_result.output_dir);
    println!("      • Files generated: {}", docs_result.files_generated);
    println!(
        "      • Patterns documented: {}",
        docs_result.patterns_documented
    );
    if let Some(index_file) = docs_result.index_file {
        println!("      • Index file: {}", index_file);
    }

    // Check if files were actually created
    if docs_dir.exists() {
        let files: Vec<_> = std::fs::read_dir(&docs_dir)?.collect();
        println!("   📄 Generated files:");
        for file in files {
            if let Ok(entry) = file {
                println!("      • {}", entry.file_name().to_string_lossy());
            }
        }
    }

    Ok(())
}

async fn import_patterns_example(scope_path: &Path) -> RhemaResult<()> {
    let json_file = scope_path.join("patterns_export.json");

    if !json_file.exists() {
        println!("   ⚠️  Skipping import - export file not found");
        return Ok(());
    }

    // Validate import file
    let validation_result = validate_import_file(&json_file.to_string_lossy(), ImportFormat::Auto)?;
    println!("   🔍 Import file validation:");
    println!("      • Valid: {}", validation_result.is_valid);
    println!("      • Errors: {}", validation_result.errors.len());
    println!("      • Warnings: {}", validation_result.warnings.len());

    if !validation_result.is_valid {
        println!("   ❌ Import file validation failed");
        return Ok(());
    }

    // Dry run import
    let dry_run_result = dry_run_import(
        scope_path,
        &json_file.to_string_lossy(),
        ImportFormat::Auto,
        ImportStrategy::Merge,
    )?;

    println!("   🔍 Dry run import results:");
    println!(
        "      • Patterns to import: {}",
        dry_run_result.patterns_count
    );
    println!(
        "      • Templates to import: {}",
        dry_run_result.templates_count
    );
    println!(
        "      • Conflicts detected: {}",
        dry_run_result.conflicts_count
    );
    println!("      • Warnings: {}", dry_run_result.warnings_count);

    // Actual import
    let import_result = import_patterns(
        scope_path,
        &json_file.to_string_lossy(),
        ImportFormat::Auto,
        ImportStrategy::Merge,
    )?;

    println!("   📥 Import results:");
    println!(
        "      • Patterns imported: {}",
        import_result.patterns_imported
    );
    println!(
        "      • Templates imported: {}",
        import_result.templates_imported
    );
    println!("      • Skipped: {}", import_result.skipped_count);
    println!("      • Errors: {}", import_result.errors.len());

    Ok(())
}

async fn analyze_patterns_example(scope_path: &Path) -> RhemaResult<()> {
    // Analyze all patterns
    let analysis = analyze_pattern_usage(scope_path, None)?;

    println!("   📊 Pattern analysis results:");
    println!("      • Total patterns: {}", analysis.total_patterns);
    println!(
        "      • Average effectiveness: {:.1}/10",
        analysis.average_effectiveness
    );
    println!(
        "      • Patterns needing review: {}",
        analysis.patterns_needing_review
    );

    if !analysis.patterns_by_category.is_empty() {
        println!("   📂 Patterns by category:");
        for (category, count) in &analysis.patterns_by_category {
            println!("      • {}: {}", category, count);
        }
    }

    if !analysis.patterns_by_maturity.is_empty() {
        println!("   🌱 Patterns by maturity:");
        for (maturity, count) in &analysis.patterns_by_maturity {
            println!("      • {}: {}", maturity, count);
        }
    }

    if !analysis.top_patterns.is_empty() {
        println!("   ⭐ Top patterns (effectiveness ≥ 8):");
        for pattern in &analysis.top_patterns {
            println!(
                "      • {} ({}): {}/10",
                pattern.name, pattern.id, pattern.effectiveness
            );
        }
    }

    if !analysis.recommendations.is_empty() {
        println!("   💡 Recommendations:");
        for rec in &analysis.recommendations {
            println!("      • {}", rec);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pattern_management_example() {
        let temp_dir = std::env::temp_dir().join("rhema_pattern_test");
        std::fs::create_dir_all(&temp_dir).unwrap();

        // Test creating patterns
        create_sample_patterns(&temp_dir).await.unwrap();

        // Test exporting patterns
        export_patterns_example(&temp_dir).await.unwrap();

        // Test validating patterns
        validate_patterns_example(&temp_dir).await.unwrap();

        // Test generating documentation
        generate_documentation_example(&temp_dir).await.unwrap();

        // Test analyzing patterns
        analyze_patterns_example(&temp_dir).await.unwrap();

        // Cleanup
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }
}
