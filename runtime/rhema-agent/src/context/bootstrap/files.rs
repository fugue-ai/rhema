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

use crate::{Rhema, RhemaResult, RhemaScope};
use std::fs;
use std::path::PathBuf;
use super::types::*;
use super::formatting::*;

/// Write bootstrap files
pub fn write_bootstrap_files(
    content: &BootstrapContent,
    output_path: &PathBuf,
    format: &str,
) -> RhemaResult<()> {
    match format.to_lowercase().as_str() {
        "json" => {
            let json_content = serde_json::to_string_pretty(content)?;
            fs::write(output_path.join("bootstrap.json"), json_content)?;
        }
        "yaml" => {
            let yaml_content = serde_yaml::to_string(content)?;
            fs::write(output_path.join("bootstrap.yaml"), yaml_content)?;
        }
        "markdown" => {
            let md_content = format_bootstrap_markdown(content);
            fs::write(output_path.join("bootstrap.md"), md_content)?;
        }
        "text" => {
            let text_content = format_bootstrap_text(content);
            fs::write(output_path.join("bootstrap.txt"), text_content)?;
        }
        "all" => {
            // Write all formats
            let json_content = serde_json::to_string_pretty(content)?;
            fs::write(output_path.join("bootstrap.json"), json_content)?;

            let yaml_content = serde_yaml::to_string(content)?;
            fs::write(output_path.join("bootstrap.yaml"), yaml_content)?;

            let md_content = format_bootstrap_markdown(content);
            fs::write(output_path.join("bootstrap.md"), md_content)?;

            let text_content = format_bootstrap_text(content);
            fs::write(output_path.join("bootstrap.txt"), text_content)?;
        }
        _ => {
            return Err(crate::RhemaError::ConfigError(format!(
                "Unsupported format: {}",
                format
            )));
        }
    }

    Ok(())
}

/// Generate bootstrap primer
pub fn generate_bootstrap_primer(
    _rhema: &Rhema,
    _scopes: &[RhemaScope],
    output_path: &PathBuf,
    use_case: &str,
) -> RhemaResult<()> {
    // This would integrate with the primer command
    // For now, create a simple primer file
    let primer_content = format!("# Rhema Bootstrap Primer - {}\n\nThis primer provides context for AI agents working on {} tasks.\n", use_case, use_case);
    fs::write(output_path.join("primer.md"), primer_content)?;

    Ok(())
}

/// Generate bootstrap README
pub fn generate_bootstrap_readme(content: &BootstrapContent, output_path: &PathBuf) -> RhemaResult<()> {
    let readme_content = format!("# Rhema Context Bootstrap\n\nThis directory contains bootstrap files for {} use case.\n\n## Files\n\n- `bootstrap.json` - JSON format bootstrap data\n- `bootstrap.yaml` - YAML format bootstrap data\n- `bootstrap.md` - Markdown format bootstrap data\n- `bootstrap.txt` - Text format bootstrap data\n- `primer.md` - Context primer for AI agents\n\n## Usage\n\nUse these files to bootstrap AI agent context for {} tasks.\n", content.use_case.name, content.use_case.name);
    fs::write(output_path.join("README.md"), readme_content)?;

    Ok(())
}
