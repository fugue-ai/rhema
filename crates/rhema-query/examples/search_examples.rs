//! Search examples for rhema-query
//!
//! This example demonstrates the search functionality of rhema-query.

use rhema_core::RhemaResult;
use rhema_query::search::{SearchEngine, SearchOptions, SearchType};
use std::collections::HashMap;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> RhemaResult<()> {
    println!("🔎 Rhema Search Examples");
    println!("=======================");

    let mut search_engine = SearchEngine::new();
    let repo_path = PathBuf::from(".");

    // Example 1: Build search index
    println!("\n1. Building Search Index");
    println!("------------------------");

    // For this example, we'll create a mock scope list
    let scopes = vec![]; // In a real scenario, this would be populated

    match search_engine.build_index(&repo_path, &scopes).await {
        Ok(()) => {
            println!("✅ Search index built successfully");
        }
        Err(e) => {
            println!("❌ Failed to build search index: {}", e);
            println!("Continuing with examples...");
        }
    }

    // Example 2: Full-text search
    println!("\n2. Full-Text Search");
    println!("-------------------");

    let full_text_options = SearchOptions {
        limit: Some(20),
        case_sensitive: false,
        fuzzy_matching: true,
        search_type: SearchType::FullText,
        filters: Vec::new(),
        semantic_weight: None,
        keyword_weight: None,
        min_similarity: None,
        fuzzy_distance: None,
        search_fields: Vec::new(),
        field_boosts: HashMap::new(),
    };

    match search_engine
        .full_text_search("JWT authentication", Some(full_text_options))
        .await
    {
        Ok(results) => {
            println!("✅ Full-text search completed");
            println!("Found {} results", results.len());
            for (i, result) in results.iter().take(3).enumerate() {
                println!("  {}. {} (score: {:.3})", i + 1, result.path, result.score);
            }
        }
        Err(e) => {
            println!("❌ Full-text search failed: {}", e);
        }
    }

    // Example 3: Regex search
    println!("\n3. Regex Search");
    println!("---------------");

    let regex_options = SearchOptions {
        limit: Some(10),
        case_sensitive: false,
        search_type: SearchType::Regex,
        filters: Vec::new(),
        semantic_weight: None,
        keyword_weight: None,
        min_similarity: None,
        fuzzy_matching: false,
        fuzzy_distance: None,
        search_fields: Vec::new(),
        field_boosts: HashMap::new(),
    };

    match search_engine
        .regex_search(r"auth.*token", Some(regex_options))
        .await
    {
        Ok(results) => {
            println!("✅ Regex search completed");
            println!("Found {} results", results.len());
            for (i, result) in results.iter().take(3).enumerate() {
                println!("  {}. {} (score: {:.3})", i + 1, result.path, result.score);
            }
        }
        Err(e) => {
            println!("❌ Regex search failed: {}", e);
        }
    }

    // Example 4: Search suggestions
    println!("\n4. Search Suggestions");
    println!("--------------------");

    match search_engine.get_suggestions("auth").await {
        Ok(suggestions) => {
            println!("✅ Search suggestions generated");
            println!("Suggestions: {:?}", suggestions);
        }
        Err(e) => {
            println!("❌ Search suggestions failed: {}", e);
        }
    }

    // Example 5: Search with filters
    println!("\n5. Search with Filters");
    println!("---------------------");

    let filter_options = SearchOptions {
        limit: Some(15),
        case_sensitive: false,
        fuzzy_matching: true,
        search_type: SearchType::FullText,
        filters: vec![rhema_query::search::SearchFilter::FileType(
            "yaml".to_string(),
        )],
        semantic_weight: None,
        keyword_weight: None,
        min_similarity: None,
        fuzzy_distance: Some(2),
        search_fields: Vec::new(),
        field_boosts: HashMap::new(),
    };

    match search_engine
        .full_text_search("authentication", Some(filter_options))
        .await
    {
        Ok(results) => {
            println!("✅ Filtered search completed");
            println!("Found {} results", results.len());
            for (i, result) in results.iter().take(3).enumerate() {
                println!("  {}. {} (score: {:.3})", i + 1, result.path, result.score);
            }
        }
        Err(e) => {
            println!("❌ Filtered search failed: {}", e);
        }
    }

    println!("\n🎉 All search examples completed!");
    Ok(())
}
