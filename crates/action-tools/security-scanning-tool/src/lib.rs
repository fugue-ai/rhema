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

use async_trait::async_trait;
use rhema_action_tool::{ActionIntent, ActionResult};
use rhema_action_tool::{SafetyTool, ToolResult};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use tokio::fs;

/// Security scanning safety tool
pub struct SecurityScanningTool;

#[async_trait]
impl SafetyTool for SecurityScanningTool {
    async fn check(&self, intent: &ActionIntent) -> ActionResult<ToolResult> {
        let start_time = std::time::Instant::now();
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut vulnerabilities = Vec::new();

        // Get the target paths from the intent scope
        if intent.scope.is_empty() {
            return Ok(ToolResult {
                success: false,
                changes: vec![],
                output: "No target paths specified for security scanning".to_string(),
                errors: vec!["No target paths specified in scope".to_string()],
                warnings: vec![],
                duration: start_time.elapsed(),
            });
        }

        // Use the first path in scope as the primary target
        let target_path = &intent.scope[0];

        // Perform multiple security scans
        let scan_results = self.perform_security_scans(target_path).await;

        for (scan_type, result) in scan_results {
            match result {
                Ok(scan_output) => {
                    if !scan_output.vulnerabilities.is_empty() {
                        let vuln_count = scan_output.vulnerabilities.len();
                        vulnerabilities.extend(scan_output.vulnerabilities);
                        warnings.push(format!(
                            "{} found {} vulnerabilities",
                            scan_type, vuln_count
                        ));
                    }
                }
                Err(e) => {
                    errors.push(format!("{} scan failed: {}", scan_type, e));
                }
            }
        }

        // Generate comprehensive report
        let output = self.generate_security_report(&vulnerabilities, &warnings, &errors);

        Ok(ToolResult {
            success: errors.is_empty(),
            changes: vec![],
            output,
            errors,
            warnings,
            duration: start_time.elapsed(),
        })
    }

    fn name(&self) -> &str {
        "security_scanning"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    async fn is_available(&self) -> bool {
        // Check if any security scanning tools are available
        self.check_security_tools_availability().await
    }
}

impl SecurityScanningTool {
    /// Perform multiple security scans on the target path
    async fn perform_security_scans(
        &self,
        target_path: &str,
    ) -> HashMap<String, Result<ScanOutput, String>> {
        let mut results = HashMap::new();

        // Dependency vulnerability scan
        if self.is_cargo_audit_available().await {
            results.insert(
                "Cargo Audit".to_string(),
                self.run_cargo_audit(target_path).await,
            );
        }

        // SAST scan with semgrep
        if self.is_semgrep_available().await {
            results.insert(
                "Semgrep SAST".to_string(),
                self.run_semgrep_scan(target_path).await,
            );
        }

        // Secret scanning
        results.insert(
            "Secret Scanning".to_string(),
            self.run_secret_scan(target_path).await,
        );

        // Dependency license scan
        results.insert(
            "License Scan".to_string(),
            self.run_license_scan(target_path).await,
        );

        results
    }

    /// Run Cargo audit for Rust dependency vulnerabilities
    async fn run_cargo_audit(&self, target_path: &str) -> Result<ScanOutput, String> {
        let output = Command::new("cargo")
            .args(&["audit", "--json"])
            .current_dir(target_path)
            .output()
            .map_err(|e| format!("Failed to run cargo audit: {}", e))?;

        let mut vulnerabilities = Vec::new();

        if output.status.success() {
            // Parse JSON output
            if let Ok(json_output) = serde_json::from_slice::<Value>(&output.stdout) {
                if let Some(advisories) = json_output.get("vulnerabilities") {
                    if let Some(advisory_list) = advisories.as_array() {
                        for advisory in advisory_list {
                            if let (Some(id), Some(title), Some(severity)) = (
                                advisory.get("id").and_then(|v| v.as_str()),
                                advisory.get("title").and_then(|v| v.as_str()),
                                advisory.get("severity").and_then(|v| v.as_str()),
                            ) {
                                vulnerabilities.push(Vulnerability {
                                    id: id.to_string(),
                                    title: title.to_string(),
                                    severity: severity.to_string(),
                                    description: advisory
                                        .get("description")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    cve: advisory
                                        .get("cve")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    source: "cargo-audit".to_string(),
                                });
                            }
                        }
                    }
                }
            }
        } else {
            let error_output = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Cargo audit failed: {}", error_output));
        }

        Ok(ScanOutput { vulnerabilities })
    }

    /// Run Semgrep SAST scan
    async fn run_semgrep_scan(&self, target_path: &str) -> Result<ScanOutput, String> {
        let output = Command::new("semgrep")
            .args(&["--json", "--config=auto", target_path])
            .output()
            .map_err(|e| format!("Failed to run semgrep: {}", e))?;

        let mut vulnerabilities = Vec::new();

        if let Ok(json_output) = serde_json::from_slice::<Value>(&output.stdout) {
            if let Some(results) = json_output.get("results").and_then(|v| v.as_array()) {
                for result in results {
                    if let (Some(check_id), Some(message), Some(severity)) = (
                        result.get("check_id").and_then(|v| v.as_str()),
                        result.get("message").and_then(|v| v.as_str()),
                        result
                            .get("extra")
                            .and_then(|v| v.get("severity"))
                            .and_then(|v| v.as_str()),
                    ) {
                        vulnerabilities.push(Vulnerability {
                            id: check_id.to_string(),
                            title: message.to_string(),
                            severity: severity.to_string(),
                            description: message.to_string(),
                            cve: "".to_string(),
                            source: "semgrep".to_string(),
                        });
                    }
                }
            }
        }

        Ok(ScanOutput { vulnerabilities })
    }

    /// Run secret scanning
    async fn run_secret_scan(&self, target_path: &str) -> Result<ScanOutput, String> {
        let mut vulnerabilities = Vec::new();

        // Simple regex-based secret scanning
        let secret_patterns = vec![
            (r"sk-[a-zA-Z0-9]{48}", "OpenAI API Key"),
            (r"ghp_[a-zA-Z0-9]{36}", "GitHub Personal Access Token"),
            (r"[a-zA-Z0-9]{40}", "GitHub OAuth Token"),
            (r"AKIA[0-9A-Z]{16}", "AWS Access Key ID"),
            (r"[0-9a-zA-Z/+]{40}", "AWS Secret Access Key"),
            (r"AIza[0-9A-Za-z\\-_]{35}", "Google API Key"),
        ];

        if let Ok(entries) = fs::read_dir(target_path).await {
            let mut entries = entries;
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_file() {
                    if let Ok(content) = fs::read_to_string(&path).await {
                        for (pattern, secret_type) in &secret_patterns {
                            if let Ok(regex) = regex::Regex::new(pattern) {
                                if regex.is_match(&content) {
                                    vulnerabilities.push(Vulnerability {
                                        id: format!(
                                            "secret-{}",
                                            secret_type.to_lowercase().replace(" ", "-")
                                        ),
                                        title: format!("Potential {} found", secret_type),
                                        severity: "high".to_string(),
                                        description: format!(
                                            "Potential {} found in file: {:?}",
                                            secret_type, path
                                        ),
                                        cve: "".to_string(),
                                        source: "secret-scan".to_string(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(ScanOutput { vulnerabilities })
    }

    /// Run license scan
    async fn run_license_scan(&self, target_path: &str) -> Result<ScanOutput, String> {
        let mut vulnerabilities = Vec::new();

        // Check for license files
        let license_files = vec!["LICENSE", "LICENSE.txt", "license.txt", "COPYING"];
        let mut found_license = false;

        for license_file in &license_files {
            let license_path = Path::new(target_path).join(license_file);
            if license_path.exists() {
                found_license = true;
                break;
            }
        }

        if !found_license {
            vulnerabilities.push(Vulnerability {
                id: "missing-license".to_string(),
                title: "Missing License File".to_string(),
                severity: "medium".to_string(),
                description: "No license file found in the project root".to_string(),
                cve: "".to_string(),
                source: "license-scan".to_string(),
            });
        }

        Ok(ScanOutput { vulnerabilities })
    }

    /// Check if security tools are available
    async fn check_security_tools_availability(&self) -> bool {
        self.is_cargo_audit_available().await || self.is_semgrep_available().await
    }

    /// Check if cargo-audit is available
    async fn is_cargo_audit_available(&self) -> bool {
        Command::new("cargo")
            .args(&["audit", "--version"])
            .output()
            .is_ok()
    }

    /// Check if semgrep is available
    async fn is_semgrep_available(&self) -> bool {
        Command::new("semgrep")
            .args(&["--version"])
            .output()
            .is_ok()
    }

    /// Generate comprehensive security report
    fn generate_security_report(
        &self,
        vulnerabilities: &[Vulnerability],
        warnings: &[String],
        errors: &[String],
    ) -> String {
        let mut report = String::new();
        report.push_str("🔒 Security Scan Report\n");
        report.push_str("=====================\n\n");

        if vulnerabilities.is_empty() && errors.is_empty() {
            report.push_str("✅ No security vulnerabilities found!\n");
        } else {
            if !vulnerabilities.is_empty() {
                report.push_str("🚨 Vulnerabilities Found:\n");
                for vuln in vulnerabilities {
                    report.push_str(&format!(
                        "  • {} ({}): {}\n",
                        vuln.severity.to_uppercase(),
                        vuln.source,
                        vuln.title
                    ));
                    if !vuln.description.is_empty() {
                        report.push_str(&format!("    Description: {}\n", vuln.description));
                    }
                    if !vuln.cve.is_empty() {
                        report.push_str(&format!("    CVE: {}\n", vuln.cve));
                    }
                    report.push('\n');
                }
            }

            if !errors.is_empty() {
                report.push_str("❌ Scan Errors:\n");
                for error in errors {
                    report.push_str(&format!("  • {}\n", error));
                }
                report.push('\n');
            }
        }

        if !warnings.is_empty() {
            report.push_str("⚠️  Warnings:\n");
            for warning in warnings {
                report.push_str(&format!("  • {}\n", warning));
            }
        }

        report
    }
}

/// Vulnerability information
#[derive(Debug, Clone)]
struct Vulnerability {
    id: String,
    title: String,
    severity: String,
    description: String,
    cve: String,
    source: String,
}

/// Scan output containing vulnerabilities
#[derive(Debug)]
struct ScanOutput {
    vulnerabilities: Vec<Vulnerability>,
}
