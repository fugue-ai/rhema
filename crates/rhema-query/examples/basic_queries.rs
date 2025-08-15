//! Basic query examples for rhema-query
//! 
//! This example demonstrates the core query functionality of rhema-query.

use rhema_query::{execute_query, execute_query_with_provenance};
use rhema_core::RhemaResult;
use std::path::PathBuf;

fn main() -> RhemaResult<()> {
    println!("🔍 Rhema Query Examples");
    println!("=======================");

    // Example 1: Basic query execution
    println!("\n1. Basic Query Execution");
    println!("------------------------");
    
    let repo_path = PathBuf::from(".");
    let query = "todos WHERE status='pending'";
    
    match execute_query(&repo_path, query) {
        Ok(results) => {
            println!("✅ Query executed successfully");
            println!("Results: {:?}", results);
        }
        Err(e) => {
            println!("❌ Query failed: {}", e);
        }
    }

    // Example 2: Query with provenance tracking
    println!("\n2. Query with Provenance Tracking");
    println!("----------------------------------");
    
    let query_with_provenance = "todos WHERE priority>5 ORDER BY created_at DESC LIMIT 10";
    
    match execute_query_with_provenance(&repo_path, query_with_provenance) {
        Ok((results, provenance)) => {
            println!("✅ Query with provenance executed successfully");
            println!("Results count: {}", results.as_sequence().map(|arr| arr.len()).unwrap_or(0));
            println!("Provenance: {:?}", provenance);
        }
        Err(e) => {
            println!("❌ Query with provenance failed: {}", e);
        }
    }

    // Example 3: YAML path query
    println!("\n3. YAML Path Query");
    println!("------------------");
    
    let yaml_path_query = "todos.title WHERE priority>3";
    
    match execute_query(&repo_path, yaml_path_query) {
        Ok(results) => {
            println!("✅ YAML path query executed successfully");
            println!("Results: {:?}", results);
        }
        Err(e) => {
            println!("❌ YAML path query failed: {}", e);
        }
    }

    // Example 4: Complex filtering
    println!("\n4. Complex Filtering");
    println!("-------------------");
    
    let complex_query = "todos WHERE status='pending' OR status='in_progress' AND priority>=3";
    
    match execute_query(&repo_path, complex_query) {
        Ok(results) => {
            println!("✅ Complex query executed successfully");
            println!("Results: {:?}", results);
        }
        Err(e) => {
            println!("❌ Complex query failed: {}", e);
        }
    }

    println!("\n🎉 All examples completed!");
    Ok(())
}
