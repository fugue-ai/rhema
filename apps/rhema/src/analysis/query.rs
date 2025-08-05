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

use crate::{Rhema, RhemaResult};
use colored::*;
use serde_yaml;

pub fn run(rhema: &Rhema, query: &str) -> RhemaResult<()> {
    println!("{}", "Executing CQL query:".bold());
    println!("  {}", query.cyan());
    println!();

    let result = rhema.query(query)?;

    // Pretty print the result
    let yaml_string = serde_yaml::to_string(&result)?;
    println!("{}", "Result:".bold());
    println!("{}", yaml_string);

    Ok(())
}

/// Execute a CQL query with statistics
pub fn run_with_stats(rhema: &Rhema, query: &str) -> RhemaResult<()> {
    println!("{}", "Executing CQL query with statistics:".bold());
    println!("  {}", query.cyan());
    println!();

    let (result, stats) = rhema.query_with_stats(query)?;

    // Print statistics first
    println!("{}", "Statistics:".bold());
    for (key, value) in &stats {
        println!("  {}: {:?}", key.green(), value);
    }
    println!();

    // Pretty print the result
    let yaml_string = serde_yaml::to_string(&result)?;
    println!("{}", "Result:".bold());
    println!("{}", yaml_string);

    Ok(())
}

/// Execute a CQL query with formatted output
pub fn run_formatted(rhema: &Rhema, query: &str, format: &str) -> RhemaResult<()> {
    println!("{}", "Executing CQL query:".bold());
    println!("  {}", query.cyan());
    println!("  Format: {}", format.yellow());
    println!();

    let result = rhema.query(query)?;

    match format.to_lowercase().as_str() {
        "yaml" | "yml" => {
            let yaml_string = serde_yaml::to_string(&result)?;
            println!("{}", yaml_string);
        }
        "json" => {
            let json_string = serde_json::to_string_pretty(&result)?;
            println!("{}", json_string);
        }
        "table" => {
            print_table_format(&result)?;
        }
        "count" => {
            if let serde_yaml::Value::Sequence(seq) = &result {
                println!("Count: {}", seq.len());
            } else {
                println!("Count: 1");
            }
        }
        _ => {
            return Err(crate::RhemaError::InvalidQuery(format!(
                "Unsupported output format: {}",
                format
            )));
        }
    }

    Ok(())
}

/// Print result in table format
fn print_table_format(result: &serde_yaml::Value) -> RhemaResult<()> {
    match result {
        serde_yaml::Value::Sequence(seq) if !seq.is_empty() => {
            // Try to extract headers from the first item
            if let Some(first_item) = seq.first() {
                if let serde_yaml::Value::Mapping(map) = first_item {
                    let headers: Vec<String> = map
                        .keys()
                        .filter_map(|k| k.as_str())
                        .map(|s| s.to_string())
                        .collect();

                    if !headers.is_empty() {
                        // Print headers
                        for header in &headers {
                            print!("{:<20} ", header.bold());
                        }
                        println!();

                        // Print separator
                        for _ in &headers {
                            print!("{:<20} ", "─".repeat(20));
                        }
                        println!();

                        // Print data rows
                        for item in seq {
                            if let serde_yaml::Value::Mapping(map) = item {
                                for header in &headers {
                                    let value = map
                                        .get(&serde_yaml::Value::String(header.clone()))
                                        .unwrap_or(&serde_yaml::Value::Null);
                                    let value_str = match value {
                                        serde_yaml::Value::String(s) => s.clone(),
                                        serde_yaml::Value::Number(n) => n.to_string(),
                                        serde_yaml::Value::Bool(b) => b.to_string(),
                                        serde_yaml::Value::Null => "null".to_string(),
                                        _ => format!("{:?}", value),
                                    };
                                    print!("{:<20} ", value_str);
                                }
                                println!();
                            }
                        }
                    } else {
                        // Fallback to simple list
                        for (i, item) in seq.iter().enumerate() {
                            println!("[{}] {:?}", i, item);
                        }
                    }
                } else {
                    // Simple list
                    for (i, item) in seq.iter().enumerate() {
                        println!("[{}] {:?}", i, item);
                    }
                }
            }
        }
        serde_yaml::Value::Sequence(_seq) => {
            println!("Empty result set");
        }
        _ => {
            println!("{:?}", result);
        }
    }

    Ok(())
}

/// Execute a CQL query with provenance tracking
pub fn run_with_provenance(rhema: &Rhema, query: &str) -> RhemaResult<()> {
    println!("{}", "Executing CQL query with provenance tracking:".bold());
    println!("  {}", query.cyan());
    println!();

    let (result, provenance) = rhema.query_with_provenance(query)?;

    // Print provenance information first
    println!("{}", "📊 Query Provenance:".bold());
    println!("{}", "─".repeat(80));

    // Basic provenance info
    println!("🔍 Original Query: {}", provenance.original_query);
    println!("⏰ Executed At: {}", provenance.executed_at);
    println!("⏱️  Execution Time: {}ms", provenance.execution_time_ms);
    println!(
        "📁 Scopes Searched: {}",
        provenance.scopes_searched.join(", ")
    );
    println!(
        "📄 Files Accessed: {}",
        provenance.files_accessed.join(", ")
    );

    // Performance metrics
    println!("\n📈 Performance Metrics:");
    println!(
        "  Total Time: {}ms",
        provenance.performance_metrics.total_time_ms
    );
    println!(
        "  Files Read: {}",
        provenance.performance_metrics.files_read
    );
    println!(
        "  YAML Documents Processed: {}",
        provenance.performance_metrics.yaml_documents_processed
    );

    // Phase times
    if !provenance.performance_metrics.phase_times.is_empty() {
        println!("  Phase Times:");
        for (phase, time) in &provenance.performance_metrics.phase_times {
            println!("    {}: {}ms", phase, time);
        }
    }

    // Execution steps
    if !provenance.execution_steps.is_empty() {
        println!("\n🔧 Execution Steps:");
        for step in &provenance.execution_steps {
            println!("  • {} ({}ms)", step.name, step.duration_ms);
        }
    }

    // Applied filters
    if !provenance.applied_filters.is_empty() {
        println!("\n🔍 Applied Filters:");
        for filter in &provenance.applied_filters {
            println!(
                "  • {}: {} ({} → {} items)",
                filter.filter_type.to_string(),
                filter.description,
                filter.items_before,
                filter.items_after
            );
        }
    }

    println!("\n{}", "📋 Query Result:".bold());
    println!("{}", "─".repeat(80));

    // Pretty print the result
    let yaml_string = serde_yaml::to_string(&result)?;
    println!("{}", yaml_string);

    Ok(())
}

/// Execute a CQL query with field-level provenance
pub fn run_with_field_provenance(rhema: &Rhema, query: &str) -> RhemaResult<()> {
    println!(
        "{}",
        "Executing CQL query with field-level provenance:".bold()
    );
    println!("  {}", query.cyan());
    println!();

    let (result, provenance) = rhema.query_with_provenance(query)?;

    // Print field-level provenance
    println!("{}", "🔍 Field-Level Provenance:".bold());
    println!("{}", "─".repeat(80));

    // For now, we'll show the basic provenance since field-level tracking
    // requires more complex result handling
    println!("📊 Query-Level Provenance:");
    println!("  Execution Time: {}ms", provenance.execution_time_ms);
    println!(
        "  Scopes Searched: {}",
        provenance.scopes_searched.join(", ")
    );
    println!("  Files Accessed: {}", provenance.files_accessed.join(", "));

    // Show applied filters
    if !provenance.applied_filters.is_empty() {
        println!("\n🔍 Applied Filters:");
        for filter in &provenance.applied_filters {
            println!(
                "  • {}: {} ({} → {} items)",
                filter.filter_type.to_string(),
                filter.description,
                filter.items_before,
                filter.items_after
            );
        }
    }

    println!("\n{}", "📋 Query Result:".bold());
    println!("{}", "─".repeat(80));

    // Pretty print the result
    let yaml_string = serde_yaml::to_string(&result)?;
    println!("{}", yaml_string);

    Ok(())
}
