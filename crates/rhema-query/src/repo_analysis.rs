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

use rhema_core::{RhemaResult, RhemaScope};
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
// use regex::Regex; // Unused import

/// Repository analysis results
#[derive(Debug, Clone)]
pub struct RepoAnalysis {
    /// Detected project type
    pub project_type: ProjectType,
    /// Primary programming languages
    pub languages: Vec<String>,
    /// Frameworks and libraries
    pub frameworks: Vec<String>,
    /// Database technologies
    pub databases: Vec<String>,
    /// Infrastructure tools
    pub infrastructure: Vec<String>,
    /// Build tools
    pub build_tools: Vec<String>,
    /// Suggested scope type
    pub suggested_scope_type: String,
    /// Suggested scope name
    pub suggested_scope_name: String,
    /// Suggested description
    pub suggested_description: String,
    /// Detected dependencies
    pub dependencies: Vec<ScopeDependency>,
    /// Custom fields based on analysis
    pub custom_fields: HashMap<String, serde_yaml::Value>,
}

/// Project type classification
#[derive(Debug, Clone)]
pub enum ProjectType {
    Monorepo,
    Microservice,
    Monolithic,
    Library,
    Application,
    Service,
    Unknown,
}

/// Scope dependency information
#[derive(Debug, Clone)]
pub struct ScopeDependency {
    pub path: String,
    pub dependency_type: String,
    pub version: Option<String>,
}

/// Technology detection patterns
#[derive(Debug, Clone)]
struct TechPattern {
    file_patterns: Vec<&'static str>,
    language: &'static str,
    frameworks: Vec<&'static str>,
}

impl RepoAnalysis {
    /// Analyze the repository and generate recommendations
    pub fn analyze(repo_path: &Path) -> RhemaResult<Self> {
        let mut analysis = RepoAnalysis {
            project_type: ProjectType::Unknown,
            languages: Vec::new(),
            frameworks: Vec::new(),
            databases: Vec::new(),
            infrastructure: Vec::new(),
            build_tools: Vec::new(),
            suggested_scope_type: "service".to_string(),
            suggested_scope_name: "unknown".to_string(),
            suggested_description: "Auto-detected scope".to_string(),
            dependencies: Vec::new(),
            custom_fields: HashMap::new(),
        };

        // Analyze file structure
        analysis.analyze_file_structure(repo_path)?;

        // Detect technologies
        analysis.detect_technologies(repo_path)?;

        // Detect databases
        analysis.detect_databases(repo_path)?;

        // Detect infrastructure
        analysis.detect_infrastructure(repo_path)?;

        // Analyze dependencies
        analysis.analyze_dependencies(repo_path)?;

        // Analyze code quality
        analysis.analyze_code_quality(repo_path)?;

        // Analyze security
        analysis.analyze_security(repo_path)?;

        // Determine project type
        analysis.determine_project_type(repo_path)?;

        // Generate recommendations
        analysis.generate_recommendations(repo_path)?;

        Ok(analysis)
    }

    /// Analyze the file structure of the repository
    fn analyze_file_structure(&mut self, repo_path: &Path) -> RhemaResult<()> {
        let mut file_extensions = HashMap::new();
        let mut directories = Vec::new();
        let mut build_files = Vec::new();

        for entry in WalkDir::new(repo_path)
            .max_depth(3) // Limit depth for performance
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() {
                // Count file extensions
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    *file_extensions.entry(ext.to_string()).or_insert(0) += 1;
                }

                // Check for build files
                let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if Self::is_build_file(file_name) {
                    build_files.push(file_name.to_string());
                }
            } else if path.is_dir() && path != repo_path {
                let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if !dir_name.starts_with('.') && dir_name != "node_modules" && dir_name != "target"
                {
                    directories.push(dir_name.to_string());
                }
            }
        }

        // Store analysis results
        self.custom_fields.insert(
            "file_extensions".to_string(),
            serde_yaml::to_value(file_extensions)?,
        );
        self.custom_fields.insert(
            "directories".to_string(),
            serde_yaml::to_value(directories)?,
        );
        self.custom_fields.insert(
            "build_files".to_string(),
            serde_yaml::to_value(build_files)?,
        );

        Ok(())
    }

    /// Detect technologies used in the project
    fn detect_technologies(&mut self, repo_path: &Path) -> RhemaResult<()> {
        // Define technology patterns
        let tech_patterns = vec![
            TechPattern {
                file_patterns: vec!["Cargo.toml", "Cargo.lock"],
                language: "Rust",
                frameworks: vec!["Actix Web", "Rocket", "Warp", "SQLx", "Diesel"],
            },
            TechPattern {
                file_patterns: vec!["package.json", "yarn.lock", "pnpm-lock.yaml"],
                language: "JavaScript/TypeScript",
                frameworks: vec!["React", "Vue", "Angular", "Express", "Next.js", "Nuxt"],
            },
            TechPattern {
                file_patterns: vec!["pom.xml", "build.gradle", "gradle.properties"],
                language: "Java",
                frameworks: vec!["Spring Boot", "Spring", "JUnit", "Maven", "Gradle"],
            },
            TechPattern {
                file_patterns: vec!["requirements.txt", "pyproject.toml", "setup.py"],
                language: "Python",
                frameworks: vec!["Django", "Flask", "FastAPI", "Pytest"],
            },
            TechPattern {
                file_patterns: vec!["go.mod", "go.sum"],
                language: "Go",
                frameworks: vec!["Gin", "Echo", "Fiber", "GORM"],
            },
            TechPattern {
                file_patterns: vec!["docker-compose.yml", "Dockerfile"],
                language: "Docker",
                frameworks: vec!["Docker Compose", "Docker Swarm"],
            },
            TechPattern {
                file_patterns: vec!["k8s/", "kubernetes/", "*.yaml", "*.yml"],
                language: "Kubernetes",
                frameworks: vec!["Kubernetes", "Helm"],
            },
        ];

        for pattern in tech_patterns {
            if Self::has_tech_pattern(repo_path, &pattern.file_patterns) {
                if !self.languages.contains(&pattern.language.to_string()) {
                    self.languages.push(pattern.language.to_string());
                }

                for framework in &pattern.frameworks {
                    if !self.frameworks.contains(&framework.to_string()) {
                        self.frameworks.push(framework.to_string());
                    }
                }
            }
        }

        // Detect databases
        self.detect_databases(repo_path)?;

        // Detect infrastructure
        self.detect_infrastructure(repo_path)?;

        Ok(())
    }

    /// Detect database technologies
    fn detect_databases(&mut self, repo_path: &Path) -> RhemaResult<()> {
        let db_patterns = vec![
            ("postgresql", vec!["postgres", "postgresql", "psql"]),
            ("mysql", vec!["mysql", "mariadb"]),
            ("sqlite", vec!["sqlite", "db.sqlite"]),
            ("redis", vec!["redis", "redis.conf"]),
            ("mongodb", vec!["mongo", "mongodb"]),
        ];

        for (db_name, patterns) in db_patterns {
            if Self::has_pattern_in_files(repo_path, &patterns) {
                self.databases.push(db_name.to_string());
            }
        }

        Ok(())
    }

    /// Detect infrastructure tools
    fn detect_infrastructure(&mut self, repo_path: &Path) -> RhemaResult<()> {
        let infra_patterns = vec![
            (
                "docker",
                vec!["Dockerfile", "docker-compose.yml", ".dockerignore"],
            ),
            ("kubernetes", vec!["k8s/", "kubernetes/", "*.yaml", "*.yml"]),
            ("terraform", vec!["*.tf", "*.tfvars", ".terraform"]),
            ("ansible", vec!["ansible.cfg", "playbook.yml", "inventory"]),
            ("jenkins", vec!["Jenkinsfile", ".jenkins"]),
            ("github-actions", vec![".github/workflows/"]),
            ("gitlab-ci", vec![".gitlab-ci.yml"]),
        ];

        for (infra_name, patterns) in infra_patterns {
            if Self::has_pattern_in_files(repo_path, &patterns) {
                self.infrastructure.push(infra_name.to_string());
            }
        }

        Ok(())
    }

    /// Determine the project type based on analysis
    fn determine_project_type(&mut self, repo_path: &Path) -> RhemaResult<()> {
        // Check for monorepo indicators
        if Self::is_monorepo(repo_path) {
            self.project_type = ProjectType::Monorepo;
            return Ok(());
        }

        // Check for microservice indicators
        if Self::is_microservice(repo_path) {
            self.project_type = ProjectType::Microservice;
            return Ok(());
        }

        // Check for library indicators
        if Self::is_library(repo_path) {
            self.project_type = ProjectType::Library;
            return Ok(());
        }

        // Check for application indicators
        if Self::is_application(repo_path) {
            self.project_type = ProjectType::Application;
            return Ok(());
        }

        // Default to service
        self.project_type = ProjectType::Service;
        Ok(())
    }

    /// Generate recommendations based on analysis
    fn generate_recommendations(&mut self, repo_path: &Path) -> RhemaResult<()> {
        // Generate scope name from directory name
        self.suggested_scope_name = repo_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Generate scope type based on project type
        self.suggested_scope_type = match self.project_type {
            ProjectType::Monorepo => "application".to_string(),
            ProjectType::Microservice => "service".to_string(),
            ProjectType::Monolithic => "application".to_string(),
            ProjectType::Library => "library".to_string(),
            ProjectType::Application => "application".to_string(),
            ProjectType::Service => "service".to_string(),
            ProjectType::Unknown => "service".to_string(),
        };

        // Generate description
        let mut description_parts = Vec::new();

        if !self.languages.is_empty() {
            description_parts.push(format!("{} project", self.languages.join("/")));
        }

        if !self.frameworks.is_empty() {
            description_parts.push(format!("using {}", self.frameworks.join(", ")));
        }

        if !self.databases.is_empty() {
            description_parts.push(format!("with {} database", self.databases.join(", ")));
        }

        match self.project_type {
            ProjectType::Monorepo => description_parts.insert(0, "Monorepo".to_string()),
            ProjectType::Microservice => description_parts.insert(0, "Microservice".to_string()),
            ProjectType::Library => description_parts.insert(0, "Library".to_string()),
            ProjectType::Application => description_parts.insert(0, "Application".to_string()),
            ProjectType::Service => description_parts.insert(0, "Service".to_string()),
            _ => {}
        }

        self.suggested_description = if description_parts.is_empty() {
            "Auto-detected scope".to_string()
        } else {
            description_parts.join(" ")
        };

        // Add tech stack to custom fields
        let mut tech_stack = serde_yaml::Mapping::new();
        tech_stack.insert(
            serde_yaml::Value::String("languages".to_string()),
            serde_yaml::to_value(&self.languages)?,
        );
        tech_stack.insert(
            serde_yaml::Value::String("frameworks".to_string()),
            serde_yaml::to_value(&self.frameworks)?,
        );
        tech_stack.insert(
            serde_yaml::Value::String("databases".to_string()),
            serde_yaml::to_value(&self.databases)?,
        );
        tech_stack.insert(
            serde_yaml::Value::String("infrastructure".to_string()),
            serde_yaml::to_value(&self.infrastructure)?,
        );
        tech_stack.insert(
            serde_yaml::Value::String("build_tools".to_string()),
            serde_yaml::to_value(&self.build_tools)?,
        );
        self.custom_fields.insert(
            "tech_stack".to_string(),
            serde_yaml::to_value(tech_stack)?,
        );

        Ok(())
    }

    /// Generate a RhemaScope from the analysis
    pub fn generate_rhema_scope(&self) -> RhemaScope {
        RhemaScope {
            name: self.suggested_scope_name.clone(),
            scope_type: self.suggested_scope_type.clone(),
            description: Some(self.suggested_description.clone()),
            version: "1.0.0".to_string(),
            schema_version: Some(rhema_core::schema::CURRENT_SCHEMA_VERSION.to_string()),
            dependencies: if self.dependencies.is_empty() {
                None
            } else {
                Some(
                    self.dependencies
                        .iter()
                        .map(|d| rhema_core::schema::ScopeDependency {
                            path: d.path.clone(),
                            dependency_type: d.dependency_type.clone(),
                            version: d.version.clone(),
                        })
                        .collect(),
                )
            },
            custom: self.custom_fields.clone(),
            protocol_info: None,
        }
    }

    // Helper methods for technology detection

    fn is_build_file(file_name: &str) -> bool {
        matches!(
            file_name,
            "Cargo.toml"
                | "package.json"
                | "pom.xml"
                | "build.gradle"
                | "requirements.txt"
                | "pyproject.toml"
                | "go.mod"
                | "Makefile"
                | "CMakeLists.txt"
                | "build.xml"
                | "gradle.properties"
        )
    }

    fn has_tech_pattern(repo_path: &Path, patterns: &[&str]) -> bool {
        for pattern in patterns {
            if Self::has_pattern_in_files(repo_path, &[pattern]) {
                return true;
            }
        }
        false
    }

    fn has_pattern_in_files(repo_path: &Path, patterns: &[&str]) -> bool {
        for entry in WalkDir::new(repo_path)
            .max_depth(2)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                let path_str = path.to_string_lossy();

                for pattern in patterns {
                    if file_name == *pattern || path_str.contains(pattern) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_monorepo(repo_path: &Path) -> bool {
        // Check for multiple package.json, Cargo.toml, etc. in subdirectories
        let mut build_file_count = 0;
        for entry in WalkDir::new(repo_path)
            .max_depth(3)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                if Self::is_build_file(file_name) {
                    build_file_count += 1;
                    if build_file_count > 1 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_microservice(repo_path: &Path) -> bool {
        // Check for microservice indicators
        let microservice_indicators = vec![
            "docker-compose.yml",
            "Dockerfile",
            "k8s/",
            "kubernetes/",
            "service.yml",
            "deployment.yml",
        ];

        Self::has_pattern_in_files(repo_path, &microservice_indicators)
    }

    fn is_library(repo_path: &Path) -> bool {
        // Check for library indicators
        let library_indicators = vec!["lib/", "src/lib.rs", "index.js", "index.ts", "__init__.py"];

        Self::has_pattern_in_files(repo_path, &library_indicators)
    }

    fn is_application(repo_path: &Path) -> bool {
        // Check for application indicators
        let app_indicators = vec![
            "src/main.rs",
            "main.py",
            "app.js",
            "app.ts",
            "index.html",
            "public/",
            "static/",
        ];

        Self::has_pattern_in_files(repo_path, &app_indicators)
    }

    /// Analyze dependencies in the repository
    fn analyze_dependencies(&mut self, repo_path: &Path) -> RhemaResult<()> {
        let mut dependencies = Vec::new();

        // Analyze Rust dependencies
        if let Ok(cargo_toml) = std::fs::read_to_string(repo_path.join("Cargo.toml")) {
            for line in cargo_toml.lines() {
                if line.trim().starts_with('[') && line.contains("dependencies") {
                    // Parse dependency section
                    dependencies.push(ScopeDependency {
                        path: "Cargo.toml".to_string(),
                        dependency_type: "rust".to_string(),
                        version: None,
                    });
                }
            }
        }

        // Analyze Node.js dependencies
        if let Ok(_package_json) = std::fs::read_to_string(repo_path.join("package.json")) {
            dependencies.push(ScopeDependency {
                path: "package.json".to_string(),
                dependency_type: "nodejs".to_string(),
                version: None,
            });
        }

        // Analyze Python dependencies
        if let Ok(_requirements_txt) = std::fs::read_to_string(repo_path.join("requirements.txt")) {
            dependencies.push(ScopeDependency {
                path: "requirements.txt".to_string(),
                dependency_type: "python".to_string(),
                version: None,
            });
        }

        self.dependencies = dependencies;
        Ok(())
    }

    /// Analyze code quality metrics
    fn analyze_code_quality(&mut self, repo_path: &Path) -> RhemaResult<()> {
        let mut quality_metrics = HashMap::new();

        // Count lines of code
        let mut total_lines = 0;
        let mut code_lines = 0;
        let mut comment_lines = 0;
        let mut blank_lines = 0;

        for entry in WalkDir::new(repo_path)
            .max_depth(5)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if matches!(ext, "rs" | "py" | "js" | "ts" | "go" | "java" | "cpp" | "c") {
                        if let Ok(content) = std::fs::read_to_string(path) {
                            for line in content.lines() {
                                total_lines += 1;
                                let trimmed = line.trim();
                                if trimmed.is_empty() {
                                    blank_lines += 1;
                                } else if trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*") {
                                    comment_lines += 1;
                                } else {
                                    code_lines += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        quality_metrics.insert("total_lines".to_string(), serde_yaml::Value::Number(total_lines.into()));
        quality_metrics.insert("code_lines".to_string(), serde_yaml::Value::Number(code_lines.into()));
        quality_metrics.insert("comment_lines".to_string(), serde_yaml::Value::Number(comment_lines.into()));
        quality_metrics.insert("blank_lines".to_string(), serde_yaml::Value::Number(blank_lines.into()));

        // Calculate code quality ratios
        if total_lines > 0 {
            let comment_ratio = comment_lines as f64 / total_lines as f64;
            let blank_ratio = blank_lines as f64 / total_lines as f64;
            let code_ratio = code_lines as f64 / total_lines as f64;

            quality_metrics.insert("comment_ratio".to_string(), serde_yaml::Value::Number(serde_yaml::Number::from(comment_ratio)));
            quality_metrics.insert("blank_ratio".to_string(), serde_yaml::Value::Number(serde_yaml::Number::from(blank_ratio)));
            quality_metrics.insert("code_ratio".to_string(), serde_yaml::Value::Number(serde_yaml::Number::from(code_ratio)));
        }

        self.custom_fields.insert("code_quality".to_string(), serde_yaml::to_value(quality_metrics)?);
        Ok(())
    }

    /// Analyze security aspects of the repository
    fn analyze_security(&mut self, repo_path: &Path) -> RhemaResult<()> {
        let mut security_analysis = HashMap::new();
        let mut security_issues = Vec::new();

        // Check for common security issues
        let security_patterns = vec![
            ("hardcoded_password", "password\\s*=\\s*['\"].*['\"]"),
            ("hardcoded_secret", "secret\\s*=\\s*['\"].*['\"]"),
            ("hardcoded_api_key", "api_key\\s*=\\s*['\"].*['\"]"),
            ("hardcoded_token", "token\\s*=\\s*['\"].*['\"]"),
            ("sql_injection_risk", "SELECT.*WHERE.*\\+"),
            ("xss_risk", "innerHTML|outerHTML"),
        ];

        for entry in WalkDir::new(repo_path)
            .max_depth(5)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if matches!(ext, "rs" | "py" | "js" | "ts" | "go" | "java" | "cpp" | "c") {
                        if let Ok(content) = std::fs::read_to_string(path) {
                            for (issue_type, pattern) in &security_patterns {
                                if let Ok(regex) = regex::Regex::new(pattern) {
                                    if regex.is_match(&content) {
                                        security_issues.push(format!("{}:{}", issue_type, path.display()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        security_analysis.insert("security_issues".to_string(), serde_yaml::Value::Sequence(
            security_issues.into_iter().map(|s| serde_yaml::Value::String(s)).collect()
        ));

        // Check for security configuration files
        let security_files = vec!["security.txt", ".security", "security.yml", "security.yaml"];
        let mut found_security_files = Vec::new();

        for file in &security_files {
            if repo_path.join(file).exists() {
                found_security_files.push(file.to_string());
            }
        }

        security_analysis.insert("security_files".to_string(), serde_yaml::Value::Sequence(
            found_security_files.into_iter().map(|s| serde_yaml::Value::String(s)).collect()
        ));

        self.custom_fields.insert("security_analysis".to_string(), serde_yaml::to_value(security_analysis)?);
        Ok(())
    }
}
