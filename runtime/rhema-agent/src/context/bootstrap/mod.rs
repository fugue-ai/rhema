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

pub mod types;
pub mod content;
pub mod formatting;
pub mod files;

use crate::{Rhema, RhemaResult, RhemaScope};
use colored::*;
use std::fs;
use std::path::PathBuf;

use self::content::*;
use self::files::*;

/// Bootstrap context for AI agents
pub fn run(
    rhema: &Rhema,
    use_case: &str,
    output_format: &str,
    output_dir: Option<&str>,
    scope_filter: Option<&str>,
    include_all: bool,
    optimize_for_ai: bool,
    create_primer: bool,
    create_readme: bool,
) -> RhemaResult<()> {
    let scopes = rhema.list_scopes()?;

    // Filter scopes if specified
    let filtered_scopes = if let Some(filter) = scope_filter {
        scopes
            .into_iter()
            .filter(|scope| {
                scope.definition.name.contains(filter)
                    || scope.definition.scope_type.contains(filter)
            })
            .collect()
    } else {
        scopes
    };

    if filtered_scopes.is_empty() {
        return Err(crate::RhemaError::ConfigError(
            "No scopes found matching the filter criteria".to_string(),
        ));
    }

    // Determine output directory
    let output_path = if let Some(dir) = output_dir {
        PathBuf::from(dir)
    } else {
        std::env::current_dir()?.join("rhema-bootstrap")
    };

    // Create output directory
    fs::create_dir_all(&output_path)?;

    // Generate bootstrap content based on use case
    // Extract RhemaScope from Scope objects
    let rhema_scopes: Vec<RhemaScope> = filtered_scopes
        .iter()
        .map(|scope| scope.definition.clone())
        .collect();

    let bootstrap_content =
        generate_bootstrap_content(rhema, &rhema_scopes, use_case, include_all, optimize_for_ai)?;

    // Write bootstrap files
    write_bootstrap_files(&bootstrap_content, &output_path, output_format)?;

    // Generate additional files if requested
    if create_primer {
        generate_bootstrap_primer(rhema, &rhema_scopes, &output_path, use_case)?;
    }

    if create_readme {
        generate_bootstrap_readme(&bootstrap_content, &output_path)?;
    }

    println!("{}", "✓ Context bootstrap completed successfully!".green());
    println!("  Use case: {}", use_case.yellow());
    println!(
        "  Output directory: {}",
        output_path.display().to_string().yellow()
    );
    println!("  Format: {}", output_format.yellow());

    Ok(())
}
