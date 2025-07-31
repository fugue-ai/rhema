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

use crate::commands::*;
use crate::{Rhema, RhemaError, RhemaResult};
// use clap::{Parser, Subcommand};
use colored::*;
// use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::io::{self, Write};
// use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

// Interactive mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveConfig {
    pub prompt: String,
    pub history_size: usize,
    pub auto_complete: bool,
    pub syntax_highlighting: bool,
    pub show_suggestions: bool,
    pub context_aware: bool,
    pub theme: Theme,
    pub keybindings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Default,
    Dark,
    Light,
    Custom {
        prompt_color: String,
        error_color: String,
        success_color: String,
        warning_color: String,
        info_color: String,
    },
}

impl Default for InteractiveConfig {
    fn default() -> Self {
        Self {
            prompt: "rhema> ".to_string(),
            history_size: 1000,
            auto_complete: true,
            syntax_highlighting: true,
            show_suggestions: true,
            context_aware: true,
            theme: Theme::Default,
            keybindings: HashMap::new(),
        }
    }
}

// Interactive session state
#[derive(Debug)]
pub struct InteractiveSession {
    rhema: Arc<Rhema>,
    config: InteractiveConfig,
    history: Vec<String>,
    history_index: usize,
    current_scope: Option<String>,
    context_cache: HashMap<String, serde_yaml::Value>,
    variables: HashMap<String, String>,
    workflows: HashMap<String, Vec<String>>,
    plugins: Vec<Box<dyn InteractivePlugin>>,
}

// Interactive plugin trait for extensibility
pub trait InteractivePlugin: Send + Sync + std::fmt::Debug {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn commands(&self) -> Vec<String>;
    fn execute(&self, session: &mut InteractiveSession, args: &[String]) -> RhemaResult<()>;
    fn suggestions(&self, session: &InteractiveSession, input: &str) -> Vec<String>;
}

// Interactive command executor
pub struct InteractiveExecutor {
    session: Arc<Mutex<InteractiveSession>>,
    _command_sender: mpsc::UnboundedSender<InteractiveCommand>,
}

#[derive(Debug)]
pub enum InteractiveCommand {
    Execute(String),
    Complete(String),
    History(HistoryDirection),
    Exit,
    Help,
    Clear,
    Config(InteractiveConfig),
}

#[derive(Debug)]
pub enum HistoryDirection {
    Up,
    Down,
}

impl InteractiveExecutor {
    pub fn new(rhema: Rhema) -> Self {
        let config = InteractiveConfig::default();
        let session = InteractiveSession::new(rhema, config);
        let session = Arc::new(Mutex::new(session));

        let (command_sender, mut command_receiver) = mpsc::unbounded_channel();

        let session_clone = session.clone();
        tokio::spawn(async move {
            while let Some(command) = command_receiver.recv().await {
                let mut session = session_clone.lock().unwrap();
                match command {
                    InteractiveCommand::Execute(input) => {
                        if let Err(e) = session.execute_command(&input) {
                            eprintln!("{}", e.to_string().red());
                        }
                    }
                    InteractiveCommand::Complete(input) => {
                        let suggestions = session.get_completions(&input);
                        if !suggestions.is_empty() {
                            println!("Suggestions: {}", suggestions.join(", ").blue());
                        }
                    }
                    InteractiveCommand::History(direction) => {
                        session.navigate_history(direction);
                    }
                    InteractiveCommand::Exit => break,
                    InteractiveCommand::Help => {
                        session.show_help();
                    }
                    InteractiveCommand::Clear => {
                        print!("\x1B[2J\x1B[1;1H");
                        io::stdout().flush().unwrap();
                    }
                    InteractiveCommand::Config(config) => {
                        session.update_config(config);
                    }
                }
            }
        });

        Self {
            session,
            _command_sender: command_sender,
        }
    }

    pub fn run(&self) -> RhemaResult<()> {
        let mut session = self.session.lock().unwrap();
        session.start_repl()
    }
}

impl InteractiveSession {
    pub fn new(rhema: Rhema, config: InteractiveConfig) -> Self {
        Self {
            rhema: Arc::new(rhema),
            config,
            history: Vec::new(),
            history_index: 0,
            current_scope: None,
            context_cache: HashMap::new(),
            variables: HashMap::new(),
            workflows: HashMap::new(),
            plugins: Vec::new(),
        }
    }

    pub fn start_repl(&mut self) -> RhemaResult<()> {
        self.show_welcome_message();
        self.show_help();

        loop {
            let input = self.read_input()?;

            if input.trim().is_empty() {
                continue;
            }

            // Add to history
            self.add_to_history(input.clone());

            // Check for special commands
            match input.trim() {
                "exit" | "quit" | "q" => break,
                "help" | "h" => self.show_help(),
                "clear" | "cls" => self.clear_screen(),
                "history" => self.show_history(),
                "config" => self.show_config(),
                "scopes" => self.list_scopes(),
                "context" => self.show_context(),
                "variables" => self.show_variables(),
                "workflows" => self.list_workflows(),
                "plugins" => self.list_plugins(),
                _ => {
                    if let Err(e) = self.execute_command(&input) {
                        eprintln!("{}", e.to_string().red());
                    }
                }
            }
        }

        println!("{}", "Goodbye!".green());
        Ok(())
    }

    fn read_input(&self) -> RhemaResult<String> {
        print!("{}", self.get_prompt());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(input.trim().to_string())
    }

    fn get_prompt(&self) -> String {
        let mut prompt = self.config.prompt.clone();

        if let Some(scope) = &self.current_scope {
            prompt = format!("rhema:{}> ", scope);
        }

        match &self.config.theme {
            Theme::Default => prompt.cyan().to_string(),
            Theme::Dark => prompt.white().on_black().to_string(),
            Theme::Light => prompt.black().on_white().to_string(),
            Theme::Custom {
                prompt_color: _, ..
            } => {
                // Apply custom color (simplified)
                prompt.cyan().to_string()
            }
        }
    }

    fn add_to_history(&mut self, input: String) {
        if !self.history.contains(&input) {
            self.history.push(input);
            if self.history.len() > self.config.history_size {
                self.history.remove(0);
            }
        }
        self.history_index = self.history.len();
    }

    fn navigate_history(&mut self, direction: HistoryDirection) {
        match direction {
            HistoryDirection::Up => {
                if self.history_index > 0 {
                    self.history_index -= 1;
                }
            }
            HistoryDirection::Down => {
                if self.history_index < self.history.len() {
                    self.history_index += 1;
                }
            }
        }
    }

    fn execute_command(&mut self, input: &str) -> RhemaResult<()> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command = parts[0];
        let args = &parts[1..];

        match command {
            "init" => self.handle_init(args),
            "scopes" => self.handle_scopes(args),
            "scope" => self.handle_scope(args),
            "tree" => self.handle_tree(),
            "show" => self.handle_show(args),
            "query" => self.handle_query(args),
            "search" => self.handle_search(args),
            "validate" => self.handle_validate(args),
            "migrate" => self.handle_migrate(args),
            "schema" => self.handle_schema(args),
            "health" => self.handle_health(args),
            "stats" => self.handle_stats(),
            "todo" => self.handle_todo(args),
            "insight" => self.handle_insight(args),
            "pattern" => self.handle_pattern(args),
            "decision" => self.handle_decision(args),
            "dependencies" => self.handle_dependencies(),
            "impact" => self.handle_impact(args),
            "sync" => self.handle_sync(),
            "git" => self.handle_git(args),
            "export" => self.handle_export(args),
            "primer" => self.handle_primer(args),
            "readme" => self.handle_readme(args),
            "bootstrap" => self.handle_bootstrap(args),
            "daemon" => self.handle_daemon(args),
            "set" => self.handle_set(args),
            "get" => self.handle_get(args),
            "workflow" => self.handle_workflow(args),
            "plugin" => self.handle_plugin(args),
            "visualize" => self.handle_visualize(args),
            "debug" => self.handle_debug(args),
            "profile" => self.handle_profile(args),
            _ => {
                // Try plugin commands
                if let Some(_plugin) = self.find_plugin(command) {
                    let _args_vec: Vec<String> = args.iter().map(|s| s.to_string()).collect();
                    // TODO: Fix borrow checker issue
                    // plugin.execute(self, &args_vec)
                    println!("Plugin execution not implemented yet");
                    Ok(())
                } else {
                    Err(RhemaError::InvalidCommand(command.to_string()))
                }
            }
        }
    }

    fn get_completions(&self, input: &str) -> Vec<String> {
        let mut completions = Vec::new();

        // Basic command completions
        let commands = vec![
            "init",
            "scopes",
            "scope",
            "tree",
            "show",
            "query",
            "search",
            "validate",
            "migrate",
            "schema",
            "health",
            "stats",
            "todo",
            "insight",
            "pattern",
            "decision",
            "dependencies",
            "impact",
            "sync",
            "git",
            "export",
            "primer",
            "readme",
            "bootstrap",
            "daemon",
            "set",
            "get",
            "workflow",
            "plugin",
            "visualize",
            "debug",
            "profile",
            "help",
            "exit",
            "clear",
            "history",
        ];

        for cmd in commands {
            if cmd.starts_with(input) {
                completions.push(cmd.to_string());
            }
        }

        // Context-aware completions
        if self.config.context_aware {
            if let Ok(scopes) = self.rhema.discover_scopes() {
                for scope in scopes {
                    if scope.definition.name.starts_with(input) {
                        completions.push(scope.definition.name);
                    }
                }
            }
        }

        // Plugin completions
        for plugin in &self.plugins {
            completions.extend(plugin.suggestions(self, input));
        }

        completions
    }

    fn show_welcome_message(&self) {
        println!("{}", "=".repeat(60).cyan());
        println!("{}", "Rhema Interactive Mode".bold().cyan());
        println!("{}", "Git-Based Agent Context Protocol".cyan());
        println!("{}", "=".repeat(60).cyan());
        println!();
        println!("Type 'help' for available commands");
        println!("Type 'exit' to quit");
        println!();
    }

    fn show_help(&self) {
        println!("{}", "Available Commands:".bold().green());
        println!();

        let commands = vec![
            (
                "Core Commands",
                vec![
                    ("init", "Initialize a new Rhema scope"),
                    ("scopes", "List all scopes in the repository"),
                    ("scope", "Show scope details"),
                    ("tree", "Show scope hierarchy tree"),
                    ("show", "Display YAML file content"),
                    ("query", "Execute a CQL query"),
                    ("search", "Search across context files"),
                ],
            ),
            (
                "Management Commands",
                vec![
                    ("validate", "Validate YAML files"),
                    ("migrate", "Migrate schema files"),
                    ("schema", "Generate schema templates"),
                    ("health", "Check scope health"),
                    ("stats", "Show context statistics"),
                ],
            ),
            (
                "Content Commands",
                vec![
                    ("todo", "Manage todo items"),
                    ("insight", "Manage knowledge insights"),
                    ("pattern", "Manage patterns"),
                    ("decision", "Manage decisions"),
                ],
            ),
            (
                "Advanced Commands",
                vec![
                    ("dependencies", "Show scope dependencies"),
                    ("impact", "Show impact of changes"),
                    ("sync", "Sync knowledge across scopes"),
                    ("git", "Advanced Git integration"),
                ],
            ),
            (
                "Export Commands",
                vec![
                    ("export", "Export context data"),
                    ("primer", "Generate context primer files"),
                    ("readme", "Generate README with context"),
                    ("bootstrap", "Bootstrap context for AI agents"),
                ],
            ),
            (
                "Interactive Commands",
                vec![
                    ("set", "Set a variable"),
                    ("get", "Get a variable"),
                    ("workflow", "Manage workflows"),
                    ("plugin", "Manage plugins"),
                    ("visualize", "Interactive data visualization"),
                    ("debug", "Debug mode"),
                    ("profile", "Performance profiling"),
                ],
            ),
            (
                "System Commands",
                vec![
                    ("help", "Show this help"),
                    ("clear", "Clear screen"),
                    ("history", "Show command history"),
                    ("config", "Show configuration"),
                    ("exit", "Exit interactive mode"),
                ],
            ),
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
        io::stdout().flush().unwrap();
    }

    fn show_history(&self) {
        println!("{}", "Command History:".bold().green());
        for (i, cmd) in self.history.iter().enumerate() {
            println!("{:3}: {}", i + 1, cmd);
        }
    }

    fn show_config(&self) {
        println!("{}", "Interactive Configuration:".bold().green());
        println!("Prompt: {}", self.config.prompt);
        println!("History Size: {}", self.config.history_size);
        println!("Auto Complete: {}", self.config.auto_complete);
        println!("Syntax Highlighting: {}", self.config.syntax_highlighting);
        println!("Show Suggestions: {}", self.config.show_suggestions);
        println!("Context Aware: {}", self.config.context_aware);
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

    fn list_plugins(&self) {
        println!("{}", "Plugins:".bold().green());
        for plugin in &self.plugins {
            println!("  {} - {}", plugin.name().cyan(), plugin.description());
        }
    }

    fn find_plugin(&self, name: &str) -> Option<&Box<dyn InteractivePlugin>> {
        self.plugins.iter().find(|p| p.name() == name)
    }

    fn update_config(&mut self, config: InteractiveConfig) {
        self.config = config;
        println!("{}", "Configuration updated".green());
    }

    // Command handlers
    fn handle_init(&mut self, args: &[&str]) -> RhemaResult<()> {
        let scope_type = args.get(0).map(|s| s.to_string());
        let scope_name = args.get(1).map(|s| s.to_string());
        let auto_config = args.contains(&"--auto-config");

        crate::init::run(
            &self.rhema,
            scope_type.as_deref(),
            scope_name.as_deref(),
            auto_config,
        )
    }

    fn handle_scopes(&mut self, _args: &[&str]) -> RhemaResult<()> {
        crate::scopes::run(&self.rhema)
    }

    fn handle_scope(&mut self, args: &[&str]) -> RhemaResult<()> {
        let path = args.get(0).map(|s| s.to_string());
        crate::scopes::show_scope(&self.rhema, path.as_deref())
    }

    fn handle_tree(&mut self) -> RhemaResult<()> {
        crate::scopes::show_tree(&self.rhema)
    }

    fn handle_show(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "show requires a file argument".to_string(),
            ));
        }

        let file = args[0].to_string();
        let scope = args.get(1).map(|s| s.to_string());

        crate::show::run(&self.rhema, &file, scope.as_deref())
    }

    fn handle_query(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "query requires a query string".to_string(),
            ));
        }

        let query = args[0].to_string();
        let stats = args.contains(&"--stats");
        let format = args
            .iter()
            .position(|&s| s == "--format")
            .and_then(|i| args.get(i + 1))
            .unwrap_or(&"yaml");

        if stats {
            crate::query::run_with_stats(&self.rhema, &query)
        } else if *format != "yaml" {
            crate::query::run_formatted(&self.rhema, &query, format)
        } else {
            crate::query::run(&self.rhema, &query)
        }
    }

    fn handle_search(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "search requires a search term".to_string(),
            ));
        }

        let term = args[0].to_string();
        let in_file = args
            .iter()
            .position(|&s| s == "--in-file")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());
        let regex = args.contains(&"--regex");

        crate::search::run(&self.rhema, &term, in_file.as_deref(), regex)
    }

    fn handle_validate(&mut self, args: &[&str]) -> RhemaResult<()> {
        let recursive = args.contains(&"--recursive");
        let json_schema = args.contains(&"--json-schema");
        let migrate = args.contains(&"--migrate");

        crate::validate::run(&self.rhema, recursive, json_schema, migrate, false, false, false)
    }

    fn handle_migrate(&mut self, args: &[&str]) -> RhemaResult<()> {
        let recursive = args.contains(&"--recursive");
        let dry_run = args.contains(&"--dry-run");

        crate::migrate::run(&self.rhema, recursive, dry_run)
    }

    fn handle_schema(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "schema requires a template type".to_string(),
            ));
        }

        let template_type = args[0].to_string();
        let output_file = args
            .iter()
            .position(|&s| s == "--output-file")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        crate::schema::run(&self.rhema, &template_type, output_file.as_deref())
    }

    fn handle_health(&mut self, args: &[&str]) -> RhemaResult<()> {
        let scope = args.get(0).map(|s| s.to_string());
        crate::health::run(&self.rhema, scope.as_deref())
    }

    fn handle_stats(&mut self) -> RhemaResult<()> {
        crate::stats::run(&self.rhema)
    }

    fn handle_todo(&mut self, _args: &[&str]) -> RhemaResult<()> {
        // Convert args to TodoSubcommands
        // This is a simplified version - in practice, you'd need to parse the args properly
        let subcommand = TodoSubcommands::List {
            status: None,
            priority: None,
            assignee: None,
        };
        crate::todo::run(&self.rhema, &subcommand)
    }

    fn handle_insight(&mut self, _args: &[&str]) -> RhemaResult<()> {
        // Similar to todo - simplified
        let subcommand = InsightSubcommands::List {
            category: None,
            tag: None,
            min_confidence: None,
        };
        crate::insight::run(&self.rhema, &subcommand)
    }

    fn handle_pattern(&mut self, _args: &[&str]) -> RhemaResult<()> {
        // Similar to todo - simplified
        let subcommand = PatternSubcommands::List {
            pattern_type: None,
            usage: None,
            min_effectiveness: None,
        };
        crate::pattern::run(&self.rhema, &subcommand)
    }

    fn handle_decision(&mut self, _args: &[&str]) -> RhemaResult<()> {
        // Similar to todo - simplified
        let subcommand = DecisionSubcommands::List {
            status: None,
            maker: None,
        };
        crate::decision::run(&self.rhema, &subcommand)
    }

    fn handle_dependencies(&mut self) -> RhemaResult<()> {
        crate::dependencies::run(&self.rhema, false, false, false, false, false, "text")
    }

    fn handle_impact(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "impact requires a file argument".to_string(),
            ));
        }

        let file = args[0].to_string();
        crate::impact::run(&self.rhema, &file)
    }

    fn handle_sync(&mut self) -> RhemaResult<()> {
        crate::sync::run(&self.rhema)
    }

    fn handle_git(&mut self, _args: &[&str]) -> RhemaResult<()> {
        // Simplified - would need proper subcommand parsing
        let subcommand = crate::git::GitSubcommands::Status;
        crate::git::run(&self.rhema, &subcommand)
    }

    fn handle_export(&mut self, args: &[&str]) -> RhemaResult<()> {
        let format = args
            .iter()
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

        let output_file = args
            .iter()
            .position(|&s| s == "--output-file")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let scope_filter = args
            .iter()
            .position(|&s| s == "--scope-filter")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        crate::export_context::run(
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
        let scope_name = args
            .iter()
            .position(|&s| s == "--scope-name")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let output_dir = args
            .iter()
            .position(|&s| s == "--output-dir")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let template_type = args
            .iter()
            .position(|&s| s == "--template-type")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let include_examples = args.contains(&"--include-examples");
        let validate = args.contains(&"--validate");

        crate::primer::run(
            &self.rhema,
            scope_name.as_deref(),
            output_dir.as_deref(),
            template_type.as_deref(),
            include_examples,
            validate,
        )
    }

    fn handle_readme(&mut self, args: &[&str]) -> RhemaResult<()> {
        let scope_name = args
            .iter()
            .position(|&s| s == "--scope-name")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let output_file = args
            .iter()
            .position(|&s| s == "--output-file")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let template = args
            .iter()
            .position(|&s| s == "--template")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let include_context = args.contains(&"--include-context");
        let seo_optimized = args.contains(&"--seo-optimized");

        let custom_sections = args
            .iter()
            .position(|&s| s == "--custom-sections")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let custom_sections_vec = custom_sections
            .as_ref()
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect());

        crate::generate_readme::run(
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
        let use_case = args
            .iter()
            .position(|&s| s == "--use-case")
            .and_then(|i| args.get(i + 1))
            .unwrap_or(&"code_review");

        let output_format = args
            .iter()
            .position(|&s| s == "--output-format")
            .and_then(|i| args.get(i + 1))
            .unwrap_or(&"json");

        let output_dir = args
            .iter()
            .position(|&s| s == "--output-dir")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let scope_filter = args
            .iter()
            .position(|&s| s == "--scope-filter")
            .and_then(|i| args.get(i + 1))
            .map(|s| s.to_string());

        let include_all = args.contains(&"--include-all");
        let optimize_for_ai = args.contains(&"--optimize-for-ai");
        let create_primer = args.contains(&"--create-primer");
        let create_readme = args.contains(&"--create-readme");

        crate::bootstrap_context::run(
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

    fn handle_daemon(&mut self, args: &[&str]) -> RhemaResult<()> {
        // Simplified - would need proper args parsing
        // Parse daemon command based on args
        let command = if args.contains(&"start") {
            crate::daemon::DaemonSubcommand::Start {
                config: args
                    .iter()
                    .position(|&s| s == "--config")
                    .and_then(|i| args.get(i + 1))
                    .map(|s| std::path::PathBuf::from(s)),
                host: "127.0.0.1".to_string(),
                port: 8080,
                unix_socket: None,
                auth: false,
                api_key: None,
                jwt_secret: None,
                redis_url: None,
                watch: false,
                watch_dirs: ".rhema".to_string(),
                log_level: "info".to_string(),
                foreground: false,
            }
        } else if args.contains(&"stop") {
            crate::daemon::DaemonSubcommand::Stop {
                pid_file: std::path::PathBuf::from("/tmp/rhema-mcp.pid"),
            }
        } else if args.contains(&"restart") {
            crate::daemon::DaemonSubcommand::Restart {
                config: args
                    .iter()
                    .position(|&s| s == "--config")
                    .and_then(|i| args.get(i + 1))
                    .map(|s| std::path::PathBuf::from(s)),
                pid_file: std::path::PathBuf::from("/tmp/rhema-mcp.pid"),
            }
        } else if args.contains(&"status") {
            crate::daemon::DaemonSubcommand::Status {
                pid_file: std::path::PathBuf::from("/tmp/rhema-mcp.pid"),
            }
        } else {
            return Err(RhemaError::InvalidCommand(
                "daemon requires a subcommand: start, stop, restart, status".to_string(),
            ));
        };

        let args = crate::daemon::DaemonArgs { command };

        tokio::runtime::Runtime::new()?.block_on(crate::daemon::execute_daemon(args))
    }

    // Interactive-specific commands
    fn handle_set(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.len() < 2 {
            return Err(RhemaError::InvalidCommand(
                "set requires key and value".to_string(),
            ));
        }

        let key = args[0].to_string();
        let value = args[1..].join(" ");
        self.variables.insert(key.clone(), value);
        println!("{} = {}", key.cyan(), self.variables[&key]);
        Ok(())
    }

    fn handle_get(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand("get requires a key".to_string()));
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
            return Err(RhemaError::InvalidCommand(
                "workflow requires a subcommand".to_string(),
            ));
        }

        match args[0] {
            "create" => {
                if args.len() < 3 {
                    return Err(RhemaError::InvalidCommand(
                        "workflow create requires name and commands".to_string(),
                    ));
                }
                let name = args[1].to_string();
                let commands: Vec<String> = args[2..].iter().map(|s| s.to_string()).collect();
                self.workflows.insert(name.clone(), commands);
                println!("{}", "Workflow created".green());
            }
            "run" => {
                if args.len() < 2 {
                    return Err(RhemaError::InvalidCommand(
                        "workflow run requires a name".to_string(),
                    ));
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
                return Err(RhemaError::InvalidCommand(
                    "Unknown workflow subcommand".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn handle_plugin(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "plugin requires a subcommand".to_string(),
            ));
        }

        match args[0] {
            "list" => {
                self.list_plugins();
            }
            "info" => {
                if args.len() < 2 {
                    return Err(RhemaError::InvalidCommand(
                        "plugin info requires a name".to_string(),
                    ));
                }
                let name = args[1];
                if let Some(plugin) = self.find_plugin(name) {
                    println!("Plugin: {}", plugin.name().cyan());
                    println!("Description: {}", plugin.description());
                    println!("Commands: {}", plugin.commands().join(", "));
                } else {
                    println!("{}", "Plugin not found".yellow());
                }
            }
            _ => {
                return Err(RhemaError::InvalidCommand(
                    "Unknown plugin subcommand".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn handle_visualize(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "visualize requires a type".to_string(),
            ));
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
                return Err(RhemaError::InvalidCommand(
                    "Unknown visualization type".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn handle_debug(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "debug requires a subcommand".to_string(),
            ));
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
                return Err(RhemaError::InvalidCommand(
                    "Unknown debug subcommand".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn handle_profile(&mut self, args: &[&str]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "profile requires a command".to_string(),
            ));
        }

        let command = args.join(" ");
        let start = std::time::Instant::now();

        let result = self.execute_command(&command);

        let duration = start.elapsed();
        println!("Command took: {:?}", duration);

        result
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
                    match crate::dependencies::run(&self.rhema, false, false, false, false, false, "text") {
            Ok(_) => println!("{}", "Dependencies visualization complete".green()),
            Err(e) => eprintln!("{}", e.to_string().red()),
        }
    }

    fn visualize_stats(&self) {
        match crate::stats::run(&self.rhema) {
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
        println!("History Size: {}", self.history.len());
        println!("Plugins Loaded: {}", self.plugins.len());
        println!("Workflows Defined: {}", self.workflows.len());
    }
}

// Built-in plugins
#[derive(Debug)]
pub struct ContextPlugin;

impl InteractivePlugin for ContextPlugin {
    fn name(&self) -> &str {
        "context"
    }

    fn description(&self) -> &str {
        "Context management and exploration"
    }

    fn commands(&self) -> Vec<String> {
        vec![
            "explore".to_string(),
            "navigate".to_string(),
            "cache".to_string(),
        ]
    }

    fn execute(&self, session: &mut InteractiveSession, args: &[String]) -> RhemaResult<()> {
        if args.is_empty() {
            return Err(RhemaError::InvalidCommand(
                "context requires a subcommand".to_string(),
            ));
        }

        match args[0].as_str() {
            "explore" => {
                println!("{}", "Context Explorer".bold().green());
                // Interactive context exploration
                Ok(())
            }
            "navigate" => {
                if args.len() < 2 {
                    return Err(RhemaError::InvalidCommand(
                        "context navigate requires a scope".to_string(),
                    ));
                }
                session.current_scope = Some(args[1].clone());
                println!("Navigated to scope: {}", args[1].cyan());
                Ok(())
            }
            "cache" => {
                println!("Cache size: {}", session.context_cache.len());
                Ok(())
            }
            _ => Err(RhemaError::InvalidCommand(
                "Unknown context subcommand".to_string(),
            )),
        }
    }

    fn suggestions(&self, _session: &InteractiveSession, input: &str) -> Vec<String> {
        if input.starts_with("context") {
            vec![
                "explore".to_string(),
                "navigate".to_string(),
                "cache".to_string(),
            ]
        } else {
            vec![]
        }
    }
}

// Main interactive mode entry point
pub fn run_interactive(rhema: Rhema) -> RhemaResult<()> {
    // Use advanced interactive mode by default
    crate::interactive_advanced::run_advanced_interactive(rhema)
}

// Interactive mode with configuration
pub fn run_interactive_with_config(
    rhema: Rhema,
    config_file: Option<&str>,
    no_auto_complete: bool,
    no_syntax_highlighting: bool,
    no_context_aware: bool,
) -> RhemaResult<()> {
    // Use advanced interactive mode with configuration
    crate::interactive_advanced::run_advanced_interactive_with_config(
        rhema,
        config_file,
        no_auto_complete,
        no_syntax_highlighting,
        no_context_aware,
    )
}
