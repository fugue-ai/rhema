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
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;
use std::io::Write;
// use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::commands::interactive_parser::InteractiveCommandParser;
// use crate::commands::interactive_builder::InteractiveBuilder;

// Advanced interactive configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedInteractiveConfig {
    pub prompt: String,
    pub history_file: Option<String>,
    pub max_history_size: usize,
    pub auto_complete: bool,
    pub syntax_highlighting: bool,
    pub show_suggestions: bool,
    pub context_aware: bool,
    pub theme: AdvancedTheme,
    pub keybindings: HashMap<String, String>,
    pub plugins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdvancedTheme {
    Default,
    Dark,
    Light,
    Custom {
        prompt_color: String,
        error_color: String,
        success_color: String,
        warning_color: String,
        info_color: String,
        suggestion_color: String,
    },
}

impl Default for AdvancedInteractiveConfig {
    fn default() -> Self {
        Self {
            prompt: "rhema> ".to_string(),
            history_file: Some("~/.rhema_history".to_string()),
            max_history_size: 10000,
            auto_complete: true,
            syntax_highlighting: true,
            show_suggestions: true,
            context_aware: true,
            theme: AdvancedTheme::Default,
            plugins: vec!["context".to_string(), "visualization".to_string()],
            keybindings: HashMap::new(),
        }
    }
}

// Advanced interactive session
#[derive(Debug)]
pub struct AdvancedInteractiveSession {
    rhema: Rhema,
    config: AdvancedInteractiveConfig,
    current_scope: Option<String>,
    context_cache: HashMap<String, serde_yaml::Value>,
    variables: HashMap<String, String>,
    workflows: HashMap<String, Vec<String>>,
    editor: Editor<(), rustyline::history::DefaultHistory>,
}

// RhemaHelper removed due to rustyline trait issues
// TODO: Re-implement when rustyline traits become public

#[allow(dead_code)]
impl AdvancedInteractiveSession {
    pub fn new(rhema: Rhema, config: AdvancedInteractiveConfig) -> RhemaResult<Self> {
        let mut editor = Editor::new()?;
        
        // Load history if configured
        if let Some(history_file) = &config.history_file {
            let history_path = shellexpand::tilde(history_file).to_string();
            let _ = editor.load_history(&history_path);
        }
        
        Ok(Self {
            rhema,
            config,
            current_scope: None,
            context_cache: HashMap::new(),
            variables: HashMap::new(),
            workflows: HashMap::new(),
            editor,
        })
    }
    
    pub fn start_repl(&mut self) -> RhemaResult<()> {
        self.show_welcome_message();
        self.show_help();
        
        loop {
            let prompt = self.get_prompt();
            let readline = self.editor.readline(&prompt);
            
            match readline {
                Ok(line) => {
                    let _ = self.editor.add_history_entry(line.as_str());
                    
                    if line.trim().is_empty() {
                        continue;
                    }
                    
                    // Check for special commands
                    match line.trim() {
                        "exit" | "quit" | "q" => break,
                        "help" | "h" => self.show_help(),
                        "clear" | "cls" => self.clear_screen(),
                        "history" => self.show_history(),
                        "config" => self.show_config(),
                        "scopes" => self.list_scopes(),
                        "context" => self.show_context(),
                        "variables" => self.show_variables(),
                        "workflows" => self.list_workflows(),
                        _ => {
                            if let Err(e) = self.execute_command(&line) {
                                eprintln!("{}", e.to_string().red());
                            }
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("^C");
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("^D");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            }
        }
        
        // Save history
        if let Some(history_file) = &self.config.history_file {
            let history_path = shellexpand::tilde(history_file).to_string();
            if let Some(parent) = std::path::Path::new(&history_path).parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let _ = self.editor.save_history(&history_path);
        }
        
        println!("{}", "Goodbye!".green());
        Ok(())
    }
    
    fn get_prompt(&self) -> String {
        let mut prompt = self.config.prompt.clone();
        
        if let Some(scope) = &self.current_scope {
            prompt = format!("rhema:{}> ", scope);
        }
        
        match &self.config.theme {
            AdvancedTheme::Default => prompt.cyan().to_string(),
            AdvancedTheme::Dark => prompt.white().on_black().to_string(),
            AdvancedTheme::Light => prompt.black().on_white().to_string(),
            AdvancedTheme::Custom { prompt_color: _, .. } => {
                // Apply custom color (simplified)
                prompt.cyan().to_string()
            }
        }
    }
    
    fn execute_command(&mut self, input: &str) -> RhemaResult<()> {
        let mut parser = InteractiveCommandParser::new(input);
        
        let command = parser.command()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("Empty command".to_string()))?;
        
        match command {
            "init" => self.handle_init_enhanced(&mut parser),
            "scopes" => self.handle_scopes_enhanced(&mut parser),
            "scope" => self.handle_scope_enhanced(&mut parser),
            "tree" => self.handle_tree_enhanced(&mut parser),
            "show" => self.handle_show_enhanced(&mut parser),
            "query" => self.handle_query_enhanced(&mut parser),
            "search" => self.handle_search_enhanced(&mut parser),
            "validate" => self.handle_validate_enhanced(&mut parser),
            "migrate" => self.handle_migrate_enhanced(&mut parser),
            "schema" => self.handle_schema_enhanced(&mut parser),
            "health" => self.handle_health_enhanced(&mut parser),
            "stats" => self.handle_stats_enhanced(&mut parser),
            "todo" => self.handle_todo_enhanced(&mut parser),
            "insight" => self.handle_insight_enhanced(&mut parser),
            "pattern" => self.handle_pattern_enhanced(&mut parser),
            "decision" => self.handle_decision_enhanced(&mut parser),
            "dependencies" => self.handle_dependencies_enhanced(&mut parser),
            "impact" => self.handle_impact_enhanced(&mut parser),
            "sync" => self.handle_sync_enhanced(&mut parser),
            "git" => self.handle_git_enhanced(&mut parser),
            "export" => self.handle_export_enhanced(&mut parser),
            "primer" => self.handle_primer_enhanced(&mut parser),
            "readme" => self.handle_readme_enhanced(&mut parser),
            "bootstrap" => self.handle_bootstrap_enhanced(&mut parser),
            "daemon" => self.handle_daemon_enhanced(&mut parser),
            "set" => self.handle_set_enhanced(&mut parser),
            "get" => self.handle_get_enhanced(&mut parser),
            "workflow" => self.handle_workflow_enhanced(&mut parser),
            "plugin" => self.handle_plugin_enhanced(&mut parser),
            "visualize" => self.handle_visualize_enhanced(&mut parser),
            "debug" => self.handle_debug_enhanced(&mut parser),
            "profile" => self.handle_profile_enhanced(&mut parser),
            "context" => self.handle_context_enhanced(&mut parser),
            "navigate" => self.handle_navigate_enhanced(&mut parser),
            "cache" => self.handle_cache_enhanced(&mut parser),
            "explore" => self.handle_explore_enhanced(&mut parser),
            "builder" => self.handle_builder_enhanced(&mut parser),
            _ => {
                let error_msg = format!("Unknown command: {}", command);
                parser.show_error_with_suggestions(&error_msg);
                Err(crate::error::RhemaError::InvalidCommand(command.to_string()))
            }
        }
    }
    
    fn show_welcome_message(&self) {
        println!("{}", "=".repeat(60).cyan());
        println!("{}", "Rhema Advanced Interactive Mode".bold().cyan());
        println!("{}", "Git-Based Agent Context Protocol".cyan());
        println!("{}", "=".repeat(60).cyan());
        println!();
        println!("Type 'help' for available commands");
        println!("Type 'exit' to quit");
        println!("Use Tab for auto-completion");
        println!("Use Ctrl+C to interrupt commands");
        println!();
    }
    
    fn show_help(&self) {
        println!("{}", "Available Commands:".bold().green());
        println!();
        
        let commands = vec![
            ("Core Commands", vec![
                ("init", "Initialize a new Rhema scope"),
                ("scopes", "List all scopes in the repository"),
                ("scope", "Show scope details"),
                ("tree", "Show scope hierarchy tree"),
                ("show", "Display YAML file content"),
                ("query", "Execute a CQL query"),
                ("search", "Search across context files"),
            ]),
            ("Management Commands", vec![
                ("validate", "Validate YAML files"),
                ("migrate", "Migrate schema files"),
                ("schema", "Generate schema templates"),
                ("health", "Check scope health"),
                ("stats", "Show context statistics"),
            ]),
            ("Content Commands", vec![
                ("todo", "Manage todo items"),
                ("insight", "Manage knowledge insights"),
                ("pattern", "Manage patterns"),
                ("decision", "Manage decisions"),
            ]),
            ("Advanced Commands", vec![
                ("dependencies", "Show scope dependencies"),
                ("impact", "Show impact of changes"),
                ("sync", "Sync knowledge across scopes"),
                ("git", "Advanced Git integration"),
            ]),
            ("Builder Commands", vec![
                ("builder todo", "Interactive todo builder"),
                ("builder insight", "Interactive insight builder"),
                ("builder pattern", "Interactive pattern builder"),
                ("builder decision", "Interactive decision builder"),
                ("builder query", "Interactive query builder"),
                ("builder prompt", "Interactive prompt pattern builder"),
            ]),
            ("Export Commands", vec![
                ("export", "Export context data"),
                ("primer", "Generate context primer files"),
                ("readme", "Generate README with context"),
                ("bootstrap", "Bootstrap context for AI agents"),
            ]),
            ("Interactive Commands", vec![
                ("set", "Set a variable"),
                ("get", "Get a variable"),
                ("workflow", "Manage workflows"),
                ("plugin", "Manage plugins"),
                ("visualize", "Interactive data visualization"),
                ("debug", "Debug mode"),
                ("profile", "Performance profiling"),
                ("context", "Context management"),
                ("navigate", "Navigate between scopes"),
                ("cache", "Cache management"),
                ("explore", "Interactive context exploration"),
            ]),
            ("System Commands", vec![
                ("help", "Show this help"),
                ("clear", "Clear screen"),
                ("history", "Show command history"),
                ("config", "Show configuration"),
                ("exit", "Exit interactive mode"),
            ]),
        ];
        
        for (category, cmds) in commands {
            println!("{}", category.bold().yellow());
            for (cmd, desc) in cmds {
                println!("  {:<15} {}", cmd.cyan(), desc);
            }
            println!();
        }
    }
    
    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().flush().unwrap();
    }
    
    fn show_history(&self) {
        println!("{}", "Command History:".bold().green());
        // History is managed by rustyline
        println!("History is automatically saved and loaded");
    }
    
    fn show_config(&self) {
        println!("{}", "Advanced Interactive Configuration:".bold().green());
        println!("Prompt: {}", self.config.prompt);
        println!("History File: {:?}", self.config.history_file);
        println!("Max History Size: {}", self.config.max_history_size);
        println!("Auto Complete: {}", self.config.auto_complete);
        println!("Syntax Highlighting: {}", self.config.syntax_highlighting);
        println!("Show Suggestions: {}", self.config.show_suggestions);
        println!("Context Aware: {}", self.config.context_aware);
        println!("Plugins: {}", self.config.plugins.join(", "));
    }
    
    fn list_scopes(&self) {
        match self.rhema.discover_scopes() {
            Ok(scopes) => {
                println!("{}", "Available Scopes:".bold().green());
                for scope in scopes {
                    println!("  {}", scope.definition.name.cyan());
                }
            }
            Err(e) => eprintln!("{}", e.to_string().red()),
        }
    }
    
    fn show_context(&self) {
        println!("{}", "Current Context:".bold().green());
        if let Some(scope) = &self.current_scope {
            println!("Current Scope: {}", scope.cyan());
        } else {
            println!("Current Scope: {}", "None".yellow());
        }
        println!("Cached Contexts: {}", self.context_cache.len());
        println!("Variables: {}", self.variables.len());
    }
    
    fn show_variables(&self) {
        println!("{}", "Variables:".bold().green());
        for (key, value) in &self.variables {
            println!("  {} = {}", key.cyan(), value);
        }
    }
    
    fn list_workflows(&self) {
        println!("{}", "Workflows:".bold().green());
        for (name, steps) in &self.workflows {
            println!("  {} ({} steps)", name.cyan(), steps.len());
        }
    }
    
    // Command handlers (simplified versions)
    #[allow(dead_code)]
    fn handle_init(&mut self, args: &[&str]) -> RhemaResult<()> {
        let scope_type = args.get(0).map(|s| s.to_string());
        let scope_name = args.get(1).map(|s| s.to_string());
        let auto_config = args.contains(&"--auto-config");
        
        crate::commands::init::run(&self.rhema, scope_type.as_deref(), scope_name.as_deref(), auto_config)
    }
    
    #[allow(dead_code)]
    fn handle_scopes(&mut self, _args: &[&str]) -> RhemaResult<()> {
        crate::commands::scopes::run(&self.rhema)
    }
    
    #[allow(dead_code)]
    fn handle_scope(&mut self, args: &[&str]) -> RhemaResult<()> {
        let path = args.get(0).map(|s| s.to_string());
        crate::commands::scopes::show_scope(&self.rhema, path.as_deref())
    }
    
    #[allow(dead_code)]
    fn handle_tree(&mut self) -> RhemaResult<()> {
        crate::commands::scopes::show_tree(&self.rhema)
    }
    
    #[allow(dead_code)]
    fn handle_show(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("show requires a file argument".to_string()));
        }
        
        let file = args[0].to_string();
        let scope = args.get(1).map(|s| s.to_string());
        
        crate::commands::show::run(&self.rhema, &file, scope.as_deref())
    }
    
    #[allow(dead_code)]
    fn handle_query(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("query requires a query string".to_string()));
        }
        
        let query = args[0].to_string();
        let stats = args.contains(&"--stats");
        let format = args.iter()
            .position(|&s| s == "--format")
            .and_then(|i| args.get(i + 1))
            .unwrap_or(&"yaml");
        
        if stats {
            crate::commands::query::run_with_stats(&self.rhema, &query)
        } else if *format != "yaml" {
            crate::commands::query::run_formatted(&self.rhema, &query, format)
        } else {
            crate::commands::query::run(&self.rhema, &query)
        }
    }
    
    #[allow(dead_code)]
    fn handle_search(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("search requires a search term".to_string()));
        }
        
        let term = args[0].to_string();
        let in_file = args.iter()
            .position(|&s| s == "--in-file")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        let regex = args.contains(&"--regex");
        
        crate::commands::search::run(&self.rhema, &term, in_file.as_deref(), regex)
    }
    
    fn handle_validate(&mut self, args: &[&str]) -> RhemaResult<()> {
        let recursive = args.contains(&"--recursive");
        let json_schema = args.contains(&"--json-schema");
        let migrate = args.contains(&"--migrate");
        
        crate::commands::validate::run(&self.rhema, recursive, json_schema, migrate)
    }
    
    fn handle_migrate(&mut self, args: &[&str]) -> RhemaResult<()> {
        let recursive = args.contains(&"--recursive");
        let dry_run = args.contains(&"--dry-run");
        
        crate::commands::migrate::run(&self.rhema, recursive, dry_run)
    }
    
    fn handle_schema(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("schema requires a template type".to_string()));
        }
        
        let template_type = args[0].to_string();
        let output_file = args.iter()
            .position(|&s| s == "--output-file")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        crate::commands::schema::run(&self.rhema, &template_type, output_file.as_deref())
    }
    
    fn handle_health(&mut self, args: &[&str]) -> RhemaResult<()> {
        let scope = args.get(0).map(|s| s.to_string());
        crate::commands::health::run(&self.rhema, scope.as_deref())
    }
    
    fn handle_stats(&mut self) -> RhemaResult<()> {
        crate::commands::stats::run(&self.rhema)
    }
    
    fn handle_todo(&mut self, _args: &[&str]) -> RhemaResult<()> {
        // Simplified - would need proper subcommand parsing
        let subcommand = crate::TodoSubcommands::List {
            status: None,
            priority: None,
            assignee: None,
        };
        crate::commands::todo::run(&self.rhema, &subcommand)
    }
    
    fn handle_insight(&mut self, _args: &[&str]) -> RhemaResult<()> {
        let subcommand = crate::InsightSubcommands::List {
            category: None,
            tag: None,
            min_confidence: None,
        };
        crate::commands::insight::run(&self.rhema, &subcommand)
    }
    
    fn handle_pattern(&mut self, _args: &[&str]) -> RhemaResult<()> {
        let subcommand = crate::PatternSubcommands::List {
            pattern_type: None,
            usage: None,
            min_effectiveness: None,
        };
        crate::commands::pattern::run(&self.rhema, &subcommand)
    }
    
    fn handle_decision(&mut self, _args: &[&str]) -> RhemaResult<()> {
        let subcommand = crate::DecisionSubcommands::List {
            status: None,
            maker: None,
        };
        crate::commands::decision::run(&self.rhema, &subcommand)
    }
    
    fn handle_dependencies(&mut self) -> RhemaResult<()> {
        crate::commands::dependencies::run(&self.rhema)
    }
    
    fn handle_impact(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("impact requires a file argument".to_string()));
        }
        
        let file = args[0].to_string();
        crate::commands::impact::run(&self.rhema, &file)
    }
    
    fn handle_sync(&mut self) -> RhemaResult<()> {
        crate::commands::sync::run(&self.rhema)
    }
    
    fn handle_git(&mut self, _args: &[&str]) -> RhemaResult<()> {
        let subcommand = crate::GitSubcommands::Status;
        crate::commands::git::run(&self.rhema, &subcommand)
    }
    
    fn handle_export(&mut self, args: &[&str]) -> RhemaResult<()> {
        let format = args.iter()
            .position(|&s| s == "--format")
            .and_then(|i| args.get(i + 1))
            .unwrap_or(&"json");
        
        let mut include_protocol = false;
        let mut include_knowledge = false;
        let mut include_todos = false;
        let mut include_decisions = false;
        let mut include_patterns = false;
        let mut include_conventions = false;
        let mut summarize = false;
        let mut ai_agent_format = false;
        
        for arg in args {
            match *arg {
                "--include-protocol" => include_protocol = true,
                "--include-knowledge" => include_knowledge = true,
                "--include-todos" => include_todos = true,
                "--include-decisions" => include_decisions = true,
                "--include-patterns" => include_patterns = true,
                "--include-conventions" => include_conventions = true,
                "--summarize" => summarize = true,
                "--ai-agent-format" => ai_agent_format = true,
                _ => {}
            }
        }
        
        let output_file = args.iter()
            .position(|&s| s == "--output-file")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let scope_filter = args.iter()
            .position(|&s| s == "--scope-filter")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        crate::commands::export_context::run(
            &self.rhema,
            format,
            output_file.as_deref(),
            scope_filter.as_deref(),
            include_protocol,
            include_knowledge,
            include_todos,
            include_decisions,
            include_patterns,
            include_conventions,
            summarize,
            ai_agent_format,
        )
    }
    
    fn handle_primer(&mut self, args: &[&str]) -> RhemaResult<()> {
        let scope_name = args.iter()
            .position(|&s| s == "--scope-name")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let output_dir = args.iter()
            .position(|&s| s == "--output-dir")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let template_type = args.iter()
            .position(|&s| s == "--template-type")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let include_examples = args.contains(&"--include-examples");
        let validate = args.contains(&"--validate");
        
        crate::commands::primer::run(
            &self.rhema,
            scope_name.as_deref(),
            output_dir.as_deref(),
            template_type.as_deref(),
            include_examples,
            validate,
        )
    }
    
    fn handle_readme(&mut self, args: &[&str]) -> RhemaResult<()> {
        let scope_name = args.iter()
            .position(|&s| s == "--scope-name")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let output_file = args.iter()
            .position(|&s| s == "--output-file")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let template = args.iter()
            .position(|&s| s == "--template")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let include_context = args.contains(&"--include-context");
        let seo_optimized = args.contains(&"--seo-optimized");
        
        let custom_sections = args.iter()
            .position(|&s| s == "--custom-sections")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let custom_sections_vec = custom_sections.as_ref().map(|s| s.split(',').map(|s| s.trim().to_string()).collect());
        
        crate::commands::generate_readme::run(
            &self.rhema,
            scope_name.as_deref(),
            output_file.as_deref(),
            template.as_deref(),
            include_context,
            seo_optimized,
            custom_sections_vec,
        )
    }
    
    fn handle_bootstrap(&mut self, args: &[&str]) -> RhemaResult<()> {
        let use_case = args.iter()
            .position(|&s| s == "--use-case")
            .and_then(|i| args.get(i + 1))
            .unwrap_or(&"code_review");
        
        let output_format = args.iter()
            .position(|&s| s == "--output-format")
            .and_then(|i| args.get(i + 1))
            .unwrap_or(&"json");
        
        let output_dir = args.iter()
            .position(|&s| s == "--output-dir")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let scope_filter = args.iter()
            .position(|&s| s == "--scope-filter")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        
        let include_all = args.contains(&"--include-all");
        let optimize_for_ai = args.contains(&"--optimize-for-ai");
        let create_primer = args.contains(&"--create-primer");
        let create_readme = args.contains(&"--create-readme");
        
        crate::commands::bootstrap_context::run(
            &self.rhema,
            use_case,
            output_format,
            output_dir.as_deref(),
            scope_filter.as_deref(),
            include_all,
            optimize_for_ai,
            create_primer,
            create_readme,
        )
    }
    
    // Old handle_daemon method removed - use handle_daemon_enhanced instead
    
    // Interactive-specific commands
    fn handle_set(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.len() < 2 {
            return Err(crate::error::RhemaError::InvalidCommand("set requires key and value".to_string()));
        }
        
        let key = args[0].to_string();
        let value = args[1..].join(" ");
        self.variables.insert(key.clone(), value);
        println!("{} = {}", key.cyan(), self.variables[&key]);
        Ok(())
    }
    
    fn handle_get(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("get requires a key".to_string()));
        }
        
        let key = args[0];
        if let Some(value) = self.variables.get(key) {
            println!("{} = {}", key.cyan(), value);
        } else {
            println!("{}", "Variable not found".yellow());
        }
        Ok(())
    }
    
    fn handle_workflow(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("workflow requires a subcommand".to_string()));
        }
        
        match args[0] {
            "create" => {
                if args.len() < 3 {
                    return Err(crate::error::RhemaError::InvalidCommand("workflow create requires name and commands".to_string()));
                }
                let name = args[1].to_string();
                let commands: Vec<String> = args[2..].iter().map(|s| s.to_string()).collect();
                self.workflows.insert(name.clone(), commands);
                println!("{}", "Workflow created".green());
            }
            "run" => {
                if args.len() < 2 {
                    return Err(crate::error::RhemaError::InvalidCommand("workflow run requires a name".to_string()));
                }
                let name = args[1];
                if let Some(commands) = self.workflows.get(name) {
                    println!("Running workflow: {}", name.cyan());
                    let commands_clone = commands.clone();
                    for cmd in commands_clone {
                        println!("  {}", cmd);
                        if let Err(e) = self.execute_command(&cmd) {
                            eprintln!("{}", e.to_string().red());
                        }
                    }
                } else {
                    println!("{}", "Workflow not found".yellow());
                }
            }
            "list" => {
                self.list_workflows();
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown workflow subcommand".to_string()));
            }
        }
        Ok(())
    }
    
    fn handle_plugin(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("plugin requires a subcommand".to_string()));
        }
        
        match args[0] {
            "list" => {
                println!("{}", "Plugins:".bold().green());
                for plugin in &self.config.plugins {
                    println!("  {}", plugin.cyan());
                }
            }
            "info" => {
                if args.len() < 2 {
                    return Err(crate::error::RhemaError::InvalidCommand("plugin info requires a name".to_string()));
                }
                let name = args[1];
                if self.config.plugins.contains(&name.to_string()) {
                    println!("Plugin: {}", name.cyan());
                    println!("Status: {}", "Loaded".green());
                } else {
                    println!("{}", "Plugin not found".yellow());
                }
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown plugin subcommand".to_string()));
            }
        }
        Ok(())
    }
    
    fn handle_visualize(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("visualize requires a type".to_string()));
        }
        
        match args[0] {
            "scopes" => {
                self.visualize_scopes();
            }
            "dependencies" => {
                self.visualize_dependencies();
            }
            "stats" => {
                self.visualize_stats();
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown visualization type".to_string()));
            }
        }
        Ok(())
    }
    
    fn handle_debug(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("debug requires a subcommand".to_string()));
        }
        
        match args[0] {
            "context" => {
                self.debug_context();
            }
            "cache" => {
                self.debug_cache();
            }
            "performance" => {
                self.debug_performance();
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown debug subcommand".to_string()));
            }
        }
        Ok(())
    }
    
    fn handle_profile(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("profile requires a command".to_string()));
        }
        
        let command = args.join(" ");
        let start = std::time::Instant::now();
        
        let result = self.execute_command(&command);
        
        let duration = start.elapsed();
        println!("Command took: {:?}", duration);
        
        result
    }
    
    // Advanced interactive commands
    fn handle_context(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("context requires a subcommand".to_string()));
        }
        
        match args[0] {
            "explore" => {
                println!("{}", "Context Explorer".bold().green());
                // Interactive context exploration
                Ok(())
            }
            "navigate" => {
                if args.len() < 2 {
                    return Err(crate::error::RhemaError::InvalidCommand("context navigate requires a scope".to_string()));
                }
                self.current_scope = Some(args[1].to_string());
                println!("Navigated to scope: {}", args[1].cyan());
                Ok(())
            }
            "cache" => {
                println!("Cache size: {}", self.context_cache.len());
                Ok(())
            }
            _ => Err(crate::error::RhemaError::InvalidCommand("Unknown context subcommand".to_string())),
        }
    }
    
    fn handle_navigate(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("navigate requires a scope".to_string()));
        }
        
        let scope = args[0];
        self.current_scope = Some(scope.to_string());
        println!("Navigated to scope: {}", scope.cyan());
        Ok(())
    }
    
    fn handle_cache(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            println!("Cache size: {}", self.context_cache.len());
            return Ok(());
        }
        
        match args[0] {
            "clear" => {
                self.context_cache.clear();
                println!("{}", "Cache cleared".green());
            }
            "list" => {
                println!("{}", "Cached contexts:".bold().green());
                for (key, _) in &self.context_cache {
                    println!("  {}", key.cyan());
                }
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown cache subcommand".to_string()));
            }
        }
        Ok(())
    }
    
    fn handle_explore(&mut self, _args: &[&str]) -> RhemaResult<()> {
        println!("{}", "Interactive Context Explorer".bold().green());
        println!("This feature allows you to explore context interactively");
        println!("Use 'context explore' for more detailed exploration");
        Ok(())
    }

    // Visualization methods
    fn visualize_scopes(&self) {
        match self.rhema.discover_scopes() {
            Ok(scopes) => {
                println!("{}", "Scope Hierarchy:".bold().green());
                for scope in scopes {
                    println!("  {}", scope.definition.name.cyan());
                    if let Ok(scope_data) = self.rhema.get_scope(&scope.definition.name) {
                        if let Some(description) = scope_data.definition.description {
                            println!("    {}", description);
                        }
                    }
                }
            }
            Err(e) => eprintln!("{}", e.to_string().red()),
        }
    }
    
    fn visualize_dependencies(&self) {
        match crate::commands::dependencies::run(&self.rhema) {
            Ok(_) => println!("{}", "Dependencies visualization complete".green()),
            Err(e) => eprintln!("{}", e.to_string().red()),
        }
    }
    
    fn visualize_stats(&self) {
        match crate::commands::stats::run(&self.rhema) {
            Ok(_) => println!("{}", "Statistics visualization complete".green()),
            Err(e) => eprintln!("{}", e.to_string().red()),
        }
    }
    
    // Debug methods
    fn debug_context(&self) {
        println!("{}", "Context Debug Information:".bold().green());
        println!("Repository Root: {:?}", self.rhema.repo_root());
        println!("Current Scope: {:?}", self.current_scope);
        println!("Context Cache Size: {}", self.context_cache.len());
        println!("Variables: {:?}", self.variables);
    }
    
    fn debug_cache(&self) {
        println!("{}", "Cache Debug Information:".bold().green());
        for (key, value) in &self.context_cache {
            println!("  {}: {:?}", key.cyan(), value);
        }
    }
    
    fn debug_performance(&self) {
        println!("{}", "Performance Debug Information:".bold().green());
        println!("Plugins Loaded: {}", self.config.plugins.len());
        println!("Workflows Defined: {}", self.workflows.len());
    }

    // Enhanced command handlers using the new parser
    fn handle_todo_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        match parser.parse_todo_subcommand() {
            Ok(subcommand) => crate::commands::todo::run(&self.rhema, &subcommand),
            Err(e) => {
                parser.show_error_with_suggestions(&e.to_string());
                Err(e)
            }
        }
    }

    fn handle_insight_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        match parser.parse_insight_subcommand() {
            Ok(subcommand) => crate::commands::insight::run(&self.rhema, &subcommand),
            Err(e) => {
                parser.show_error_with_suggestions(&e.to_string());
                Err(e)
            }
        }
    }

    fn handle_pattern_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        match parser.parse_pattern_subcommand() {
            Ok(subcommand) => crate::commands::pattern::run(&self.rhema, &subcommand),
            Err(e) => {
                parser.show_error_with_suggestions(&e.to_string());
                Err(e)
            }
        }
    }

    fn handle_decision_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        match parser.parse_decision_subcommand() {
            Ok(subcommand) => crate::commands::decision::run(&self.rhema, &subcommand),
            Err(e) => {
                parser.show_error_with_suggestions(&e.to_string());
                Err(e)
            }
        }
    }

    fn handle_git_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        match parser.parse_git_subcommand() {
            Ok(subcommand) => crate::commands::git::run(&self.rhema, &subcommand),
            Err(e) => {
                parser.show_error_with_suggestions(&e.to_string());
                Err(e)
            }
        }
    }

    // Simple enhanced handlers for other commands
    fn handle_init_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let scope_type = parser.next().map(|s| s.to_string());
        let scope_name = parser.next().map(|s| s.to_string());
        let auto_config = parser.remaining().iter().any(|arg| arg == "--auto-config");
        
        crate::commands::init::run(&self.rhema, scope_type.as_deref(), scope_name.as_deref(), auto_config)
    }

    fn handle_scopes_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        crate::commands::scopes::run(&self.rhema)
    }

    fn handle_scope_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let path = parser.next().map(|s| s.to_string());
        crate::commands::scopes::show_scope(&self.rhema, path.as_deref())
    }

    fn handle_tree_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        crate::commands::scopes::show_tree(&self.rhema)
    }

    fn handle_show_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let file = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("show requires a file argument".to_string()))?
            .to_string();
        let scope = parser.next().map(|s| s.to_string());
        
        crate::commands::show::run(&self.rhema, &file, scope.as_deref())
    }

    fn handle_query_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let query = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("query requires a query string".to_string()))?
            .to_string();
        
        let mut stats = false;
        let mut format = "yaml".to_string();
        
        while let Some(arg) = parser.next() {
            match arg {
                "--stats" => stats = true,
                "--format" => {
                    format = parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--format requires a value".to_string()))?
                        .to_string();
                }
                _ => {
                    return Err(crate::error::RhemaError::InvalidCommand(
                        format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        
        if stats {
            crate::commands::query::run_with_stats(&self.rhema, &query)
        } else if format != "yaml" {
            crate::commands::query::run_formatted(&self.rhema, &query, &format)
        } else {
            crate::commands::query::run(&self.rhema, &query)
        }
    }

    fn handle_search_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let term = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("search requires a search term".to_string()))?
            .to_string();
        
        let mut in_file = None;
        let mut regex = false;
        
        while let Some(arg) = parser.next() {
            match arg {
                "--in-file" => {
                    in_file = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--in-file requires a value".to_string()))?
                        .to_string());
                }
                "--regex" => regex = true,
                _ => {
                    return Err(crate::error::RhemaError::InvalidCommand(
                        format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        
        crate::commands::search::run(&self.rhema, &term, in_file.as_deref(), regex)
    }

    fn handle_validate_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let mut recursive = false;
        let mut json_schema = false;
        let mut migrate = false;
        
        while let Some(arg) = parser.next() {
            match arg {
                "--recursive" => recursive = true,
                "--json-schema" => json_schema = true,
                "--migrate" => migrate = true,
                _ => {
                    return Err(crate::error::RhemaError::InvalidCommand(
                        format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        
        crate::commands::validate::run(&self.rhema, recursive, json_schema, migrate)
    }

    fn handle_migrate_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let mut recursive = false;
        let mut dry_run = false;
        
        while let Some(arg) = parser.next() {
            match arg {
                "--recursive" => recursive = true,
                "--dry-run" => dry_run = true,
                _ => {
                    return Err(crate::error::RhemaError::InvalidCommand(
                        format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        
        crate::commands::migrate::run(&self.rhema, recursive, dry_run)
    }

    fn handle_schema_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let template_type = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("schema requires a template type".to_string()))?
            .to_string();
        
        let mut output_file = None;
        
        while let Some(arg) = parser.next() {
            match arg {
                "--output-file" => {
                    output_file = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--output-file requires a value".to_string()))?
                        .to_string());
                }
                _ => {
                    return Err(crate::error::RhemaError::InvalidCommand(
                        format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        
        crate::commands::schema::run(&self.rhema, &template_type, output_file.as_deref())
    }

    fn handle_health_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let scope = parser.next().map(|s| s.to_string());
        crate::commands::health::run(&self.rhema, scope.as_deref())
    }

    fn handle_stats_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        crate::commands::stats::run(&self.rhema)
    }

    fn handle_dependencies_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        crate::commands::dependencies::run(&self.rhema)
    }

    fn handle_impact_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let file = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("impact requires a file argument".to_string()))?
            .to_string();
        
        crate::commands::impact::run(&self.rhema, &file)
    }

    fn handle_sync_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        crate::commands::sync::run(&self.rhema)
    }

    fn handle_export_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let mut format = "json".to_string();
        let mut output_file = None;
        let mut scope_filter = None;
        let mut include_protocol = false;
        let mut include_knowledge = false;
        let mut include_todos = false;
        let mut include_decisions = false;
        let mut include_patterns = false;
        let mut include_conventions = false;
        let mut summarize = false;
        let mut ai_agent_format = false;
        
        while let Some(arg) = parser.next() {
            match arg {
                "--format" => {
                    format = parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--format requires a value".to_string()))?
                        .to_string();
                }
                "--output-file" => {
                    output_file = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--output-file requires a value".to_string()))?
                        .to_string());
                }
                "--scope-filter" => {
                    scope_filter = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--scope-filter requires a value".to_string()))?
                        .to_string());
                }
                "--include-protocol" => include_protocol = true,
                "--include-knowledge" => include_knowledge = true,
                "--include-todos" => include_todos = true,
                "--include-decisions" => include_decisions = true,
                "--include-patterns" => include_patterns = true,
                "--include-conventions" => include_conventions = true,
                "--summarize" => summarize = true,
                "--ai-agent-format" => ai_agent_format = true,
                _ => {
                    return Err(crate::error::RhemaError::InvalidCommand(
                        format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        
        crate::commands::export_context::run(
            &self.rhema,
            &format,
            output_file.as_deref(),
            scope_filter.as_deref(),
            include_protocol,
            include_knowledge,
            include_todos,
            include_decisions,
            include_patterns,
            include_conventions,
            summarize,
            ai_agent_format,
        )
    }

    fn handle_primer_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let mut scope_name = None;
        let mut output_dir = None;
        let mut template_type = None;
        let mut include_examples = false;
        let mut validate = false;
        
        while let Some(arg) = parser.next() {
            match arg {
                "--scope-name" => {
                    scope_name = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--scope-name requires a value".to_string()))?
                        .to_string());
                }
                "--output-dir" => {
                    output_dir = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--output-dir requires a value".to_string()))?
                        .to_string());
                }
                "--template-type" => {
                    template_type = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--template-type requires a value".to_string()))?
                        .to_string());
                }
                "--include-examples" => include_examples = true,
                "--validate" => validate = true,
                _ => {
                    return Err(crate::error::RhemaError::InvalidCommand(
                        format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        
        crate::commands::primer::run(
            &self.rhema,
            scope_name.as_deref(),
            output_dir.as_deref(),
            template_type.as_deref(),
            include_examples,
            validate,
        )
    }

    fn handle_readme_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let mut scope_name = None;
        let mut output_file = None;
        let mut template = None;
        let mut include_context = false;
        let mut seo_optimized = false;
        let mut custom_sections = None;
        
        while let Some(arg) = parser.next() {
            match arg {
                "--scope-name" => {
                    scope_name = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--scope-name requires a value".to_string()))?
                        .to_string());
                }
                "--output-file" => {
                    output_file = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--output-file requires a value".to_string()))?
                        .to_string());
                }
                "--template" => {
                    template = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--template requires a value".to_string()))?
                        .to_string());
                }
                "--include-context" => include_context = true,
                "--seo-optimized" => seo_optimized = true,
                "--custom-sections" => {
                    custom_sections = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--custom-sections requires a value".to_string()))?
                        .to_string());
                }
                _ => {
                    return Err(crate::error::RhemaError::InvalidCommand(
                        format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        
        let custom_sections_vec = custom_sections.as_ref().map(|s| s.split(',').map(|s| s.trim().to_string()).collect());
        
        crate::commands::generate_readme::run(
            &self.rhema,
            scope_name.as_deref(),
            output_file.as_deref(),
            template.as_deref(),
            include_context,
            seo_optimized,
            custom_sections_vec,
        )
    }

    fn handle_bootstrap_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let mut use_case = "code_review".to_string();
        let mut output_format = "json".to_string();
        let mut output_dir = None;
        let mut scope_filter = None;
        let mut include_all = false;
        let mut optimize_for_ai = false;
        let mut create_primer = false;
        let mut create_readme = false;
        
        while let Some(arg) = parser.next() {
            match arg {
                "--use-case" => {
                    use_case = parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--use-case requires a value".to_string()))?
                        .to_string();
                }
                "--output-format" => {
                    output_format = parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--output-format requires a value".to_string()))?
                        .to_string();
                }
                "--output-dir" => {
                    output_dir = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--output-dir requires a value".to_string()))?
                        .to_string());
                }
                "--scope-filter" => {
                    scope_filter = Some(parser.next()
                        .ok_or_else(|| crate::error::RhemaError::InvalidCommand("--scope-filter requires a value".to_string()))?
                        .to_string());
                }
                "--include-all" => include_all = true,
                "--optimize-for-ai" => optimize_for_ai = true,
                "--create-primer" => create_primer = true,
                "--create-readme" => create_readme = true,
                _ => {
                    return Err(crate::error::RhemaError::InvalidCommand(
                        format!("Unknown argument: {}", arg)
                    ));
                }
            }
        }
        
        crate::commands::bootstrap_context::run(
            &self.rhema,
            &use_case,
            &output_format,
            output_dir.as_deref(),
            scope_filter.as_deref(),
            include_all,
            optimize_for_ai,
            create_primer,
            create_readme,
        )
    }

    fn handle_daemon_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let _subcommand = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("daemon requires a subcommand".to_string()))?;
        
        // For now, just show available subcommands
        println!("{}", "Available daemon subcommands:".yellow());
        println!("  start - Start the MCP daemon");
        println!("  stop - Stop the MCP daemon");
        println!("  restart - Restart the MCP daemon");
        println!("  status - Check daemon status");
        println!("  health - Get daemon health");
        println!("  stats - Get daemon statistics");
        println!("  config - Generate configuration file");
        println!();
        println!("Use 'rhema daemon <subcommand>' directly for daemon operations");
        
        Ok(())
    }

    fn handle_set_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let key = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("set requires a key".to_string()))?
            .to_string();
        
        let value = parser.remaining().join(" ");
        if value.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("set requires a value".to_string()));
        }
        
        self.variables.insert(key.clone(), value.clone());
        println!("{} = {}", key.cyan(), value);
        Ok(())
    }

    fn handle_get_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let key = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("get requires a key".to_string()))?;
        
        if let Some(value) = self.variables.get(key) {
            println!("{} = {}", key.cyan(), value);
        } else {
            println!("{}", "Variable not found".yellow());
        }
        Ok(())
    }

    fn handle_workflow_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let subcommand = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("workflow requires a subcommand".to_string()))?;
        
        match subcommand {
            "create" => {
                let name = parser.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("workflow create requires a name".to_string()))?
                    .to_string();
                
                let commands: Vec<String> = parser.remaining().iter().map(|s| s.to_string()).collect();
                if commands.is_empty() {
                    return Err(crate::error::RhemaError::InvalidCommand("workflow create requires commands".to_string()));
                }
                
                self.workflows.insert(name.clone(), commands);
                println!("{}", "Workflow created".green());
            }
            "run" => {
                let name = parser.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("workflow run requires a name".to_string()))?;
                
                if let Some(commands) = self.workflows.get(name) {
                    println!("Running workflow: {}", name.cyan());
                    let commands_clone = commands.clone();
                    for cmd in commands_clone {
                        println!("  {}", cmd);
                        if let Err(e) = self.execute_command(&cmd) {
                            eprintln!("{}", e.to_string().red());
                        }
                    }
                } else {
                    println!("{}", "Workflow not found".yellow());
                }
            }
            "list" => {
                self.list_workflows();
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown workflow subcommand".to_string()));
            }
        }
        Ok(())
    }

    fn handle_plugin_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let subcommand = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("plugin requires a subcommand".to_string()))?;
        
        match subcommand {
            "list" => {
                // TODO: Implement list_plugins
                println!("{}", "Plugin listing not implemented yet".yellow());
            }
            "info" => {
                let _name = parser.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("plugin info requires a name".to_string()))?;
                
                // TODO: Implement find_plugin
                println!("{}", "Plugin finding not implemented yet".yellow());
                println!("{}", "Plugin not found".yellow());
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown plugin subcommand".to_string()));
            }
        }
        Ok(())
    }

    fn handle_visualize_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let visualization_type = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("visualize requires a type".to_string()))?;
        
        match visualization_type {
            "scopes" => {
                self.visualize_scopes();
            }
            "dependencies" => {
                self.visualize_dependencies();
            }
            "stats" => {
                self.visualize_stats();
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown visualization type".to_string()));
            }
        }
        Ok(())
    }

    fn handle_debug_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let subcommand = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("debug requires a subcommand".to_string()))?;
        
        match subcommand {
            "context" => {
                self.debug_context();
            }
            "cache" => {
                self.debug_cache();
            }
            "performance" => {
                self.debug_performance();
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown debug subcommand".to_string()));
            }
        }
        Ok(())
    }

    fn handle_profile_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let command = parser.remaining().join(" ");
        if command.is_empty() {
            return Err(crate::error::RhemaError::InvalidCommand("profile requires a command".to_string()));
        }
        
        let start = std::time::Instant::now();
        let result = self.execute_command(&command);
        let duration = start.elapsed();
        println!("Command took: {:?}", duration);
        
        result
    }

    fn handle_context_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let subcommand = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("context requires a subcommand".to_string()))?;
        
        match subcommand {
            "explore" => {
                println!("{}", "Context Explorer".bold().green());
                // Interactive context exploration
                Ok(())
            }
            "navigate" => {
                let scope = parser.next()
                    .ok_or_else(|| crate::error::RhemaError::InvalidCommand("context navigate requires a scope".to_string()))?;
                
                self.current_scope = Some(scope.to_string());
                println!("Navigated to scope: {}", scope.cyan());
                Ok(())
            }
            "cache" => {
                println!("Cache size: {}", self.context_cache.len());
                Ok(())
            }
            _ => {
                Err(crate::error::RhemaError::InvalidCommand("Unknown context subcommand".to_string()))
            }
        }
    }

    fn handle_navigate_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let scope = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("navigate requires a scope".to_string()))?;
        
        self.current_scope = Some(scope.to_string());
        println!("Navigated to scope: {}", scope.cyan());
        Ok(())
    }

    fn handle_cache_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let subcommand = parser.next()
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("cache requires a subcommand".to_string()))?;
        
        match subcommand {
            "clear" => {
                self.context_cache.clear();
                println!("{}", "Cache cleared".green());
            }
            "show" => {
                println!("Cache size: {}", self.context_cache.len());
                for (key, _value) in &self.context_cache {
                    println!("  {}", key.cyan());
                }
            }
            _ => {
                return Err(crate::error::RhemaError::InvalidCommand("Unknown cache subcommand".to_string()));
            }
        }
        Ok(())
    }

    fn handle_explore_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Context Explorer".bold().green());
        // Interactive context exploration
        Ok(())
    }

    fn handle_builder_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        let args = parser.args();
        let subcommand = args
            .get(1)
            .ok_or_else(|| crate::error::RhemaError::InvalidCommand("builder requires a subcommand".to_string()))?;
        
        match subcommand.as_str() {
            "todo" => {
                let builder = crate::commands::interactive_builder::InteractiveBuilder::new(self.rhema.clone());
                let command = builder.build_todo_add()?;
                println!("{}", "Execute this command:".bold().green());
                println!("{}", command);
            }
            "insight" => {
                let builder = crate::commands::interactive_builder::InteractiveBuilder::new(self.rhema.clone());
                let command = builder.build_insight_record()?;
                println!("{}", "Execute this command:".bold().green());
                println!("{}", command);
            }
            "pattern" => {
                let builder = crate::commands::interactive_builder::InteractiveBuilder::new(self.rhema.clone());
                let command = builder.build_pattern_add()?;
                println!("{}", "Execute this command:".bold().green());
                println!("{}", command);
            }
            "decision" => {
                let builder = crate::commands::interactive_builder::InteractiveBuilder::new(self.rhema.clone());
                let command = builder.build_decision_record()?;
                println!("{}", "Execute this command:".bold().green());
                println!("{}", command);
            }
            "query" => {
                let builder = crate::commands::interactive_builder::InteractiveBuilder::new(self.rhema.clone());
                let command = builder.build_query()?;
                println!("{}", "Execute this command:".bold().green());
                println!("{}", command);
            }
            "prompt" => {
                let builder = crate::commands::interactive_builder::InteractiveBuilder::new(self.rhema.clone());
                let command = builder.build_prompt_pattern()?;
                println!("{}", "Execute this command:".bold().green());
                println!("{}", command);
            }
            _ => {
                println!("{}", "Unknown builder subcommand".red());
                println!("Available subcommands: todo, insight, pattern, decision, query, prompt");
            }
        }
        
        Ok(())
    }

    fn prompt_yes_no(&self) -> RhemaResult<bool> {
        use std::io::{self, Write};
        
        loop {
            print!("{} (y/n): ", "Execute".cyan());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            let input = input.trim().to_lowercase();
            match input.as_str() {
                "y" | "yes" => return Ok(true),
                "n" | "no" => return Ok(false),
                _ => println!("{}", "Please enter 'y' or 'n'".red()),
            }
        }
    }
}

// Advanced interactive mode entry point
pub fn run_advanced_interactive(rhema: Rhema) -> RhemaResult<()> {
    let config = AdvancedInteractiveConfig::default();
    let mut session = AdvancedInteractiveSession::new(rhema, config)?;
    session.start_repl()
}

// Advanced interactive mode with configuration
pub fn run_advanced_interactive_with_config(
    rhema: Rhema,
    config_file: Option<&str>,
    no_auto_complete: bool,
    no_syntax_highlighting: bool,
    no_context_aware: bool,
) -> RhemaResult<()> {
    let mut config = if let Some(config_path) = config_file {
        // Load configuration from file
        let config_content = std::fs::read_to_string(config_path)?;
        serde_yaml::from_str(&config_content).unwrap_or_else(|_| AdvancedInteractiveConfig::default())
    } else {
        AdvancedInteractiveConfig::default()
    };
    
    // Override configuration based on command line arguments
    if no_auto_complete {
        config.auto_complete = false;
    }
    if no_syntax_highlighting {
        config.syntax_highlighting = false;
    }
    if no_context_aware {
        config.context_aware = false;
    }
    
    let mut session = AdvancedInteractiveSession::new(rhema, config)?;
    session.start_repl()
} 