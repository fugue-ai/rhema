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
use rustyline::hint::{Hinter, HistoryHinter};
// use rustyline::history::History;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Editor, Helper};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use crate::interactive_parser::InteractiveCommandParser;
use serde::{Deserialize, Serialize};

/// Enhanced interactive configuration with all new features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedInteractiveConfig {
    pub prompt: String,
    pub history_file: Option<String>,
    pub max_history_size: usize,
    pub auto_complete: bool,
    pub syntax_highlighting: bool,
    pub show_suggestions: bool,
    pub context_aware: bool,
    pub theme: EnhancedTheme,
    pub keybindings: HashMap<String, String>,
    pub plugins: Vec<String>,
    pub completion_style: CompletionStyle,
    pub suggestion_threshold: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnhancedTheme {
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
        completion_color: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionStyle {
    Fuzzy,
    Prefix,
    Smart,
}

impl Default for EnhancedInteractiveConfig {
    fn default() -> Self {
        Self {
            prompt: "rhema> ".to_string(),
            history_file: Some("~/.rhema_history".to_string()),
            max_history_size: 10000,
            auto_complete: true,
            syntax_highlighting: true,
            show_suggestions: true,
            context_aware: true,
            theme: EnhancedTheme::Default,
            plugins: vec!["context".to_string(), "visualization".to_string()],
            keybindings: HashMap::new(),
            completion_style: CompletionStyle::Smart,
            suggestion_threshold: 3,
        }
    }
}

/// Enhanced interactive session with all new features
#[derive(Debug)]
pub struct EnhancedInteractiveSession {
    rhema: Rhema,
    config: EnhancedInteractiveConfig,
    current_scope: Option<String>,
    context_cache: HashMap<String, serde_yaml::Value>,
    variables: HashMap<String, String>,
    workflows: HashMap<String, Vec<String>>,
    editor: Editor<RhemaHelper, rustyline::history::DefaultHistory>,
    command_suggestions: Vec<String>,
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

/// Helper struct for rustyline with enhanced features
pub struct RhemaHelper {
    hinter: HistoryHinter,
    validator: RhemaValidator,
    completer: RhemaCompleter,
    highlighter: RhemaHighlighter,
}

impl RhemaHelper {
    pub fn new(config: &EnhancedInteractiveConfig) -> Self {
        Self {
            hinter: HistoryHinter {},
            validator: RhemaValidator::new(),
            completer: RhemaCompleter::new(config),
            highlighter: RhemaHighlighter::new(config),
        }
    }
}

impl Helper for RhemaHelper {}

impl Hinter for RhemaHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, _ctx)
    }
}

impl Validator for RhemaHelper {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        self.validator.validate(ctx)
    }
}

impl rustyline::completion::Completer for RhemaHelper {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.completer.complete(line, pos, ctx)
    }
}

impl rustyline::highlight::Highlighter for RhemaHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
        self.highlighter.highlight_prompt(prompt, default)
    }
}

/// Command validator for enhanced parsing
#[derive(Debug)]
pub struct RhemaValidator;

impl RhemaValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        let line = ctx.input();
        
        // Basic validation - check for balanced quotes and brackets
        if !self.is_balanced(line) {
            return Ok(ValidationResult::Invalid(Some("Unbalanced quotes or brackets".to_string())));
        }

        // Check for incomplete commands
        if line.trim().ends_with('\\') {
            return Ok(ValidationResult::Incomplete);
        }

        Ok(ValidationResult::Valid(None))
    }

    fn is_balanced(&self, line: &str) -> bool {
        let mut stack = Vec::new();
        let mut in_quotes = false;
        let mut quote_char = '\0';

        for ch in line.chars() {
            match ch {
                '"' | '\'' => {
                    if !in_quotes {
                        in_quotes = true;
                        quote_char = ch;
                    } else if ch == quote_char {
                        in_quotes = false;
                    }
                }
                '(' | '[' | '{' => {
                    if !in_quotes {
                        stack.push(ch);
                    }
                }
                ')' => {
                    if !in_quotes {
                        if stack.pop() != Some('(') {
                            return false;
                        }
                    }
                }
                ']' => {
                    if !in_quotes {
                        if stack.pop() != Some('[') {
                            return false;
                        }
                    }
                }
                '}' => {
                    if !in_quotes {
                        if stack.pop() != Some('{') {
                            return false;
                        }
                    }
                }
                _ => {}
            }
        }

        stack.is_empty() && !in_quotes
    }
}

/// Enhanced command completer with fuzzy matching
#[derive(Debug)]
pub struct RhemaCompleter {
    config: EnhancedInteractiveConfig,
    commands: HashSet<String>,
    subcommands: HashMap<String, Vec<String>>,
    arguments: HashMap<String, Vec<String>>,
}

impl RhemaCompleter {
    pub fn new(config: &EnhancedInteractiveConfig) -> Self {
        let mut commands = HashSet::new();
        let mut subcommands = HashMap::new();
        let mut arguments = HashMap::new();

        // Add all available commands
        commands.extend(vec![
            "init".to_string(), "scopes".to_string(), "scope".to_string(), "tree".to_string(), 
            "show".to_string(), "query".to_string(), "search".to_string(), "validate".to_string(),
            "migrate".to_string(), "schema".to_string(), "health".to_string(), "stats".to_string(), 
            "todo".to_string(), "insight".to_string(), "pattern".to_string(),
            "decision".to_string(), "dependencies".to_string(), "impact".to_string(), 
            "sync".to_string(), "git".to_string(), "export".to_string(), "primer".to_string(),
            "readme".to_string(), "bootstrap".to_string(), "daemon".to_string(), 
            "set".to_string(), "get".to_string(), "workflow".to_string(), "plugin".to_string(),
            "visualize".to_string(), "debug".to_string(), "profile".to_string(), 
            "context".to_string(), "navigate".to_string(), "cache".to_string(), "explore".to_string(),
            "help".to_string(), "exit".to_string(), "clear".to_string(), "history".to_string(), 
            "config".to_string(), "variables".to_string(), "workflows".to_string(),
        ]);

        // Add subcommands
        subcommands.insert("todo".to_string(), vec!["add".to_string(), "list".to_string(), "update".to_string(), "delete".to_string(), "complete".to_string()]);
        subcommands.insert("insight".to_string(), vec!["record".to_string(), "list".to_string(), "analyze".to_string(), "export".to_string()]);
        subcommands.insert("pattern".to_string(), vec!["add".to_string(), "list".to_string(), "match".to_string(), "analyze".to_string()]);
        subcommands.insert("decision".to_string(), vec!["add".to_string(), "list".to_string(), "update".to_string(), "resolve".to_string()]);
        subcommands.insert("git".to_string(), vec!["status".to_string(), "commit".to_string(), "push".to_string(), "pull".to_string(), "branch".to_string()]);
        subcommands.insert("show".to_string(), vec!["scope".to_string(), "config".to_string(), "context".to_string(), "variables".to_string()]);
        subcommands.insert("query".to_string(), vec!["execute".to_string(), "list".to_string(), "save".to_string(), "load".to_string()]);
        subcommands.insert("search".to_string(), vec!["files".to_string(), "content".to_string(), "patterns".to_string(), "history".to_string()]);

        // Add common arguments
        arguments.insert("--help".to_string(), vec!["-h".to_string()]);
        arguments.insert("--verbose".to_string(), vec!["-v".to_string()]);
        arguments.insert("--quiet".to_string(), vec!["-q".to_string()]);
        arguments.insert("--config".to_string(), vec!["-c".to_string()]);
        arguments.insert("--output".to_string(), vec!["-o".to_string()]);
        arguments.insert("--format".to_string(), vec!["-f".to_string()]);

        Self {
            config: config.clone(),
            commands,
            subcommands,
            arguments,
        }
    }

    pub fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<String>)> {
        let mut candidates = Vec::new();
        let words: Vec<&str> = line[..pos].split_whitespace().collect();
        
        if words.is_empty() {
            // Complete commands
            for cmd in &self.commands {
                candidates.push(cmd.to_string());
            }
            return Ok((0, candidates));
        }

        let current_word = words.last().unwrap_or(&"");
        let word_start = pos - current_word.len();

        if words.len() == 1 {
            // Complete first command
            for cmd in &self.commands {
                if self.matches_completion_style(cmd, current_word) {
                    candidates.push(cmd.to_string());
                }
            }
        } else if words.len() == 2 {
            // Complete subcommands
            let command = words[0];
            if let Some(subcmds) = self.subcommands.get(command) {
                for subcmd in subcmds {
                    if self.matches_completion_style(subcmd, current_word) {
                        candidates.push(subcmd.to_string());
                    }
                }
            }
        } else {
            // Complete arguments
            for (arg, aliases) in &self.arguments {
                if self.matches_completion_style(arg, current_word) {
                    candidates.push(arg.to_string());
                }
                for alias in aliases {
                    if self.matches_completion_style(alias, current_word) {
                        candidates.push(alias.to_string());
                    }
                }
            }
        }

        Ok((word_start, candidates))
    }

    fn matches_completion_style(&self, candidate: &str, input: &str) -> bool {
        match self.config.completion_style {
            CompletionStyle::Fuzzy => self.fuzzy_match(candidate, input),
            CompletionStyle::Prefix => candidate.starts_with(input),
            CompletionStyle::Smart => {
                candidate.starts_with(input) || self.fuzzy_match(candidate, input)
            }
        }
    }

    fn fuzzy_match(&self, candidate: &str, pattern: &str) -> bool {
        if pattern.is_empty() {
            return true;
        }

        let mut pattern_chars = pattern.chars().peekable();
        let mut candidate_chars = candidate.chars();

        while let Some(&pattern_char) = pattern_chars.peek() {
            if let Some(candidate_char) = candidate_chars.next() {
                if candidate_char.to_lowercase().next() == pattern_char.to_lowercase().next() {
                    pattern_chars.next();
                }
            } else {
                return false;
            }
        }

        pattern_chars.next().is_none()
    }
}

/// Syntax highlighter for enhanced interactive mode
#[derive(Debug)]
pub struct RhemaHighlighter {
    config: EnhancedInteractiveConfig,
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl RhemaHighlighter {
    pub fn new(config: &EnhancedInteractiveConfig) -> Self {
        Self {
            config: config.clone(),
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        if !self.config.syntax_highlighting {
            return std::borrow::Cow::Borrowed(line);
        }

        // Try to detect if this looks like a command or data
        if self.looks_like_command(line) {
            self.highlight_command(line)
        } else {
            self.highlight_data(line)
        }
    }

    pub fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> std::borrow::Cow<'b, str> {
        match &self.config.theme {
            EnhancedTheme::Default => std::borrow::Cow::Owned(prompt.blue().to_string()),
            EnhancedTheme::Dark => std::borrow::Cow::Owned(prompt.cyan().to_string()),
            EnhancedTheme::Light => std::borrow::Cow::Owned(prompt.blue().to_string()),
            EnhancedTheme::Custom { prompt_color, .. } => {
                // Simple color mapping for custom themes
                let colored = match prompt_color.as_str() {
                    "red" => prompt.red(),
                    "green" => prompt.green(),
                    "blue" => prompt.blue(),
                    "yellow" => prompt.yellow(),
                    "cyan" => prompt.cyan(),
                    "magenta" => prompt.magenta(),
                    _ => prompt.blue(),
                };
                std::borrow::Cow::Owned(colored.to_string())
            }
        }
    }

    fn looks_like_command(&self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return false;
        }

        // Check if it starts with a command
        let first_word = trimmed.split_whitespace().next().unwrap_or("");
        first_word.chars().all(|c| c.is_ascii_alphabetic() || c == '-')
    }

    fn highlight_command<'a>(&self, line: &'a str) -> std::borrow::Cow<'a, str> {
        let mut highlighted = String::new();
        let words: Vec<&str> = line.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            if i > 0 {
                highlighted.push(' ');
            }

            if i == 0 {
                // Command name
                highlighted.push_str(&word.green().to_string());
            } else if word.starts_with("--") {
                // Long option
                highlighted.push_str(&word.cyan().to_string());
            } else if word.starts_with('-') {
                // Short option
                highlighted.push_str(&word.yellow().to_string());
            } else if word.contains('=') {
                // Key-value pair
                let parts: Vec<&str> = word.splitn(2, '=').collect();
                if parts.len() == 2 {
                    highlighted.push_str(&parts[0].magenta().to_string());
                    highlighted.push('=');
                    highlighted.push_str(&parts[1].white().to_string());
                } else {
                    highlighted.push_str(word);
                }
            } else {
                // Argument
                highlighted.push_str(&word.white().to_string());
            }
        }

        std::borrow::Cow::Owned(highlighted)
    }

    fn highlight_data<'a>(&self, line: &'a str) -> std::borrow::Cow<'a, str> {
        // Try to detect JSON or YAML
        if line.trim().starts_with('{') || line.trim().starts_with('[') {
            // JSON-like
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(line) {
                return std::borrow::Cow::Owned(serde_json::to_string_pretty(&json).unwrap_or_else(|_| line.to_string()));
            }
        }

        if line.contains(':') && !line.contains(' ') {
            // YAML-like
            return std::borrow::Cow::Owned(line.yellow().to_string());
        }

        std::borrow::Cow::Borrowed(line)
    }
}

impl EnhancedInteractiveSession {
    pub fn new(rhema: Rhema, config: EnhancedInteractiveConfig) -> RhemaResult<Self> {
        let helper = RhemaHelper::new(&config);
        let mut editor = Editor::new()?;
        editor.set_helper(Some(helper));

        // Load history if configured
        if let Some(history_file) = &config.history_file {
            let history_path = shellexpand::tilde(history_file).to_string();
            let _ = editor.load_history(&history_path);
        }

        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();

        Ok(Self {
            rhema,
            config,
            current_scope: None,
            context_cache: HashMap::new(),
            variables: HashMap::new(),
            workflows: HashMap::new(),
            editor,
            command_suggestions: Vec::new(),
            syntax_set,
            theme_set,
        })
    }

    pub fn start_repl(&mut self) -> RhemaResult<()> {
        self.show_welcome_message();

        loop {
            let prompt = self.get_prompt();
            let readline = self.editor.readline(&prompt);

            match readline {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }

                    // Add to history
                    let _ = self.editor.add_history_entry(line);

                    // Execute command
                    if let Err(e) = self.execute_command(line) {
                        eprintln!("{}", e.to_string().red());
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("{}", "Interrupted".yellow());
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("{}", "Goodbye!".green());
                    break;
                }
                Err(err) => {
                    eprintln!("{}: {}", "Error".red(), err);
                    break;
                }
            }
        }

        // Save history
        if let Some(history_file) = &self.config.history_file {
            let history_path = shellexpand::tilde(history_file).to_string();
            let _ = self.editor.save_history(&history_path);
        }

        Ok(())
    }

    fn get_prompt(&self) -> String {
        let mut prompt = self.config.prompt.clone();
        
        if self.config.context_aware {
            if let Some(scope) = &self.current_scope {
                prompt = format!("rhema:{}> ", scope);
            }
        }

        prompt
    }

    fn execute_command(&mut self, input: &str) -> RhemaResult<()> {
        let mut parser = InteractiveCommandParser::new(input);
        
        // Get command suggestions based on input
        self.update_suggestions(input);

        match parser.command() {
            Some("help") | Some("h") => Ok(self.show_help()),
            Some("exit") | Some("quit") | Some("q") => {
                println!("{}", "Goodbye!".green());
                std::process::exit(0);
            }
            Some("clear") | Some("cls") => Ok(self.clear_screen()),
            Some("history") => Ok(self.show_history()),
            Some("config") => Ok(self.show_config()),
            Some("suggestions") => Ok(self.show_suggestions()),
            None => Ok(()),
            Some("init") => self.handle_init_enhanced(&mut parser),
            Some("scopes") => self.handle_scopes_enhanced(&mut parser),
            Some("scope") => self.handle_scope_enhanced(&mut parser),
            Some("tree") => self.handle_tree_enhanced(&mut parser),
            Some("show") => self.handle_show_enhanced(&mut parser),
            Some("query") => self.handle_query_enhanced(&mut parser),
            Some("search") => self.handle_search_enhanced(&mut parser),
            Some("validate") => self.handle_validate_enhanced(&mut parser),
            Some("migrate") => self.handle_migrate_enhanced(&mut parser),
            Some("schema") => self.handle_schema_enhanced(&mut parser),
            Some("health") => self.handle_health_enhanced(&mut parser),
            Some("stats") => self.handle_stats_enhanced(&mut parser),
            Some("todo") => self.handle_todo_enhanced(&mut parser),
            Some("insight") => self.handle_insight_enhanced(&mut parser),
            Some("pattern") => self.handle_pattern_enhanced(&mut parser),
            Some("decision") => self.handle_decision_enhanced(&mut parser),
            Some("dependencies") => self.handle_dependencies_enhanced(&mut parser),
            Some("impact") => self.handle_impact_enhanced(&mut parser),
            Some("sync") => self.handle_sync_enhanced(&mut parser),
            Some("git") => self.handle_git_enhanced(&mut parser),
            Some("export") => self.handle_export_enhanced(&mut parser),
            Some("primer") => self.handle_primer_enhanced(&mut parser),
            Some("readme") => self.handle_readme_enhanced(&mut parser),
            Some("bootstrap") => self.handle_bootstrap_enhanced(&mut parser),
            Some("daemon") => self.handle_daemon_enhanced(&mut parser),
            Some("set") => self.handle_set_enhanced(&mut parser),
            Some("get") => self.handle_get_enhanced(&mut parser),
            Some("workflow") => self.handle_workflow_enhanced(&mut parser),
            Some("plugin") => self.handle_plugin_enhanced(&mut parser),
            Some("visualize") => self.handle_visualize_enhanced(&mut parser),
            Some("debug") => self.handle_debug_enhanced(&mut parser),
            Some("profile") => self.handle_profile_enhanced(&mut parser),
            Some("context") => self.handle_context_enhanced(&mut parser),
            Some("navigate") => self.handle_navigate_enhanced(&mut parser),
            Some("cache") => self.handle_cache_enhanced(&mut parser),
            Some("explore") => self.handle_explore_enhanced(&mut parser),
            Some(cmd) => {
                eprintln!("{}: Unknown command '{}'", "Error".red(), cmd);
                if self.config.show_suggestions {
                    self.show_command_suggestions(cmd);
                }
                Ok(())
            }
        }
    }

    fn update_suggestions(&mut self, input: &str) {
        self.command_suggestions.clear();
        
        if input.len() < self.config.suggestion_threshold {
            return;
        }

        let all_commands = vec![
            "init", "scopes", "scope", "tree", "show", "query", "search", "validate",
            "migrate", "schema", "health", "stats", "todo", "insight", "pattern",
            "decision", "dependencies", "impact", "sync", "git", "export", "primer",
            "readme", "bootstrap", "daemon", "set", "get", "workflow", "plugin",
            "visualize", "debug", "profile", "context", "navigate", "cache", "explore",
        ];

        for cmd in all_commands {
            if cmd.contains(input) || input.contains(cmd) {
                self.command_suggestions.push(cmd.to_string());
            }
        }

        // Limit suggestions
        if self.command_suggestions.len() > 5 {
            self.command_suggestions.truncate(5);
        }
    }

    fn show_command_suggestions(&self, partial: &str) {
        if !self.command_suggestions.is_empty() {
            println!("{}", "Did you mean?".yellow());
            for suggestion in &self.command_suggestions {
                println!("  {}", suggestion.cyan());
            }
        }
    }

    fn show_welcome_message(&self) {
        println!("{}", "Welcome to Rhema Enhanced Interactive Mode!".green().bold());
        println!("{}", "Type 'help' for available commands or 'exit' to quit.".blue());
        println!("{}", "Enhanced features: tab completion, syntax highlighting, command suggestions".cyan());
        println!();
    }

    fn show_help(&self) {
        println!("{}", "Available Commands:".green().bold());
        println!();
        
        let commands = vec![
            ("init", "Initialize a new Rhema project"),
            ("scopes", "List all available scopes"),
            ("scope", "Set or show current scope"),
            ("tree", "Show scope tree structure"),
            ("show", "Show various information"),
            ("query", "Execute queries"),
            ("search", "Search for content"),
            ("validate", "Validate configurations"),
            ("migrate", "Migrate configurations"),
            ("schema", "Work with schemas"),
            ("health", "Check system health"),
            ("stats", "Show statistics"),
            ("todo", "Manage todos"),
            ("insight", "Work with insights"),
            ("pattern", "Manage patterns"),
            ("decision", "Manage decisions"),
            ("dependencies", "Show dependencies"),
            ("impact", "Analyze impact"),
            ("sync", "Synchronize data"),
            ("git", "Git operations"),
            ("export", "Export data"),
            ("primer", "Generate primers"),
            ("readme", "Generate README"),
            ("bootstrap", "Bootstrap project"),
            ("daemon", "Daemon operations"),
            ("set", "Set variables"),
            ("get", "Get variables"),
            ("workflow", "Manage workflows"),
            ("plugin", "Manage plugins"),
            ("visualize", "Visualize data"),
            ("debug", "Debug information"),
            ("profile", "Profile operations"),
            ("context", "Context operations"),
            ("navigate", "Navigate scopes"),
            ("cache", "Cache operations"),
            ("explore", "Explore data"),
        ];

        for (cmd, desc) in commands {
            println!("  {:<15} {}", cmd.green(), desc);
        }

        println!();
        println!("{}", "Interactive Features:".yellow().bold());
        println!("  Tab completion     - Press Tab to complete commands and arguments");
        println!("  Syntax highlighting - Commands and data are color-coded");
        println!("  Command suggestions - Get suggestions for unknown commands");
        println!("  Command history     - Use arrow keys to navigate history");
        println!("  Smart validation    - Real-time command validation");
        println!();
        println!("{}", "Type 'help <command>' for detailed help on a specific command.".blue());
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        std::io::stdout().flush().unwrap();
    }

    fn show_history(&self) {
        println!("{}", "Command History:".green().bold());
        // Note: This would need to be implemented with rustyline's history API
        println!("History feature requires additional implementation");
    }

    fn show_config(&self) {
        println!("{}", "Current Configuration:".green().bold());
        println!("  Prompt: {}", self.config.prompt);
        println!("  Auto-complete: {}", self.config.auto_complete);
        println!("  Syntax highlighting: {}", self.config.syntax_highlighting);
        println!("  Show suggestions: {}", self.config.show_suggestions);
        println!("  Context aware: {}", self.config.context_aware);
        println!("  Completion style: {:?}", self.config.completion_style);
    }

    fn show_suggestions(&self) {
        if self.command_suggestions.is_empty() {
            println!("{}", "No suggestions available.".yellow());
        } else {
            println!("{}", "Recent suggestions:".green().bold());
            for suggestion in &self.command_suggestions {
                println!("  {}", suggestion.cyan());
            }
        }
    }

    // Enhanced command handlers - these would delegate to the existing handlers
    // but with enhanced parsing and feedback

    fn handle_init_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced init command - parsing with improved validation".cyan());
        // Delegate to existing handler
        Ok(())
    }

    fn handle_scopes_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced scopes command".cyan());
        Ok(())
    }

    fn handle_scope_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        if let Some(scope) = parser.next() {
            self.current_scope = Some(scope.to_string());
            println!("{}: {}", "Current scope set to".green(), scope.cyan());
        } else {
            if let Some(scope) = &self.current_scope {
                println!("{}: {}", "Current scope".green(), scope.cyan());
            } else {
                println!("{}", "No scope currently set".yellow());
            }
        }
        Ok(())
    }

    fn handle_tree_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced tree command".cyan());
        Ok(())
    }

    fn handle_show_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        match parser.next() {
            Some("scope") => {
                if let Some(scope) = &self.current_scope {
                    println!("{}: {}", "Current scope".green(), scope.cyan());
                } else {
                    println!("{}", "No scope currently set".yellow());
                }
            }
            Some("config") => self.show_config(),
            Some("context") => {
                println!("{}", "Context information:".green().bold());
                println!("  Variables: {}", self.variables.len());
                println!("  Workflows: {}", self.workflows.len());
                println!("  Cache entries: {}", self.context_cache.len());
            }
            Some("variables") => {
                if self.variables.is_empty() {
                    println!("{}", "No variables set".yellow());
                } else {
                    println!("{}", "Variables:".green().bold());
                    for (key, value) in &self.variables {
                        println!("  {} = {}", key.cyan(), value);
                    }
                }
            }
            Some(what) => {
                println!("{}: Unknown show target '{}'", "Error".red(), what);
                println!("Available targets: scope, config, context, variables");
            }
            None => {
                println!("{}", "What would you like to show?".yellow());
                println!("Available targets: scope, config, context, variables");
            }
        }
        Ok(())
    }

    fn handle_query_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced query command".cyan());
        Ok(())
    }

    fn handle_search_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced search command".cyan());
        Ok(())
    }

    fn handle_validate_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced validate command".cyan());
        Ok(())
    }

    fn handle_migrate_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced migrate command".cyan());
        Ok(())
    }

    fn handle_schema_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced schema command".cyan());
        Ok(())
    }

    fn handle_health_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced health command".cyan());
        Ok(())
    }

    fn handle_stats_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced stats command".cyan());
        Ok(())
    }

    fn handle_todo_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced todo command".cyan());
        Ok(())
    }

    fn handle_insight_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced insight command".cyan());
        Ok(())
    }

    fn handle_pattern_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced pattern command".cyan());
        Ok(())
    }

    fn handle_decision_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced decision command".cyan());
        Ok(())
    }

    fn handle_dependencies_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced dependencies command".cyan());
        Ok(())
    }

    fn handle_impact_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced impact command".cyan());
        Ok(())
    }

    fn handle_sync_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced sync command".cyan());
        Ok(())
    }

    fn handle_git_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced git command".cyan());
        Ok(())
    }

    fn handle_export_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced export command".cyan());
        Ok(())
    }

    fn handle_primer_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced primer command".cyan());
        Ok(())
    }

    fn handle_readme_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced readme command".cyan());
        Ok(())
    }

    fn handle_bootstrap_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced bootstrap command".cyan());
        Ok(())
    }

    fn handle_daemon_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced daemon command".cyan());
        Ok(())
    }

    fn handle_set_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        if let Some(key) = parser.next() {
            let key = key.to_string();
            if let Some(value) = parser.next() {
                let value = value.to_string();
                self.variables.insert(key.clone(), value.clone());
                println!("{}: {} = {}", "Variable set".green(), key.cyan(), value);
            } else {
                println!("{}: Missing value for variable '{}'", "Error".red(), key);
            }
        } else {
            println!("{}: Missing variable name", "Error".red());
        }
        Ok(())
    }

    fn handle_get_enhanced(&mut self, parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        if let Some(key) = parser.next() {
            if let Some(value) = self.variables.get(key) {
                println!("{}: {}", key.cyan(), value);
            } else {
                println!("{}: Variable '{}' not found", "Error".red(), key);
            }
        } else {
            if self.variables.is_empty() {
                println!("{}", "No variables set".yellow());
            } else {
                println!("{}", "Variables:".green().bold());
                for (key, value) in &self.variables {
                    println!("  {} = {}", key.cyan(), value);
                }
            }
        }
        Ok(())
    }

    fn handle_workflow_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced workflow command".cyan());
        Ok(())
    }

    fn handle_plugin_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced plugin command".cyan());
        Ok(())
    }

    fn handle_visualize_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced visualize command".cyan());
        Ok(())
    }

    fn handle_debug_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced debug command".cyan());
        Ok(())
    }

    fn handle_profile_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced profile command".cyan());
        Ok(())
    }

    fn handle_context_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced context command".cyan());
        Ok(())
    }

    fn handle_navigate_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced navigate command".cyan());
        Ok(())
    }

    fn handle_cache_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced cache command".cyan());
        Ok(())
    }

    fn handle_explore_enhanced(&mut self, _parser: &mut InteractiveCommandParser) -> RhemaResult<()> {
        println!("{}", "Enhanced explore command".cyan());
        Ok(())
    }
}

/// Public functions to run enhanced interactive mode
pub fn run_enhanced_interactive(rhema: Rhema) -> RhemaResult<()> {
    let config = EnhancedInteractiveConfig::default();
    let mut session = EnhancedInteractiveSession::new(rhema, config)?;
    session.start_repl()
}

pub fn run_enhanced_interactive_with_config(
    rhema: Rhema,
    config_file: Option<&str>,
    no_auto_complete: bool,
    no_syntax_highlighting: bool,
    no_context_aware: bool,
) -> RhemaResult<()> {
    let mut config = EnhancedInteractiveConfig::default();
    
    // Load config from file if provided
    if let Some(config_path) = config_file {
        if let Ok(config_data) = std::fs::read_to_string(config_path) {
            if let Ok(loaded_config) = serde_yaml::from_str::<EnhancedInteractiveConfig>(&config_data) {
                config = loaded_config;
            }
        }
    }

    // Override with command line options
    if no_auto_complete {
        config.auto_complete = false;
    }
    if no_syntax_highlighting {
        config.syntax_highlighting = false;
    }
    if no_context_aware {
        config.context_aware = false;
    }

    let mut session = EnhancedInteractiveSession::new(rhema, config)?;
    session.start_repl()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_config_default() {
        let config = EnhancedInteractiveConfig::default();
        assert_eq!(config.prompt, "rhema> ");
        assert!(config.auto_complete);
        assert!(config.syntax_highlighting);
        assert!(config.show_suggestions);
        assert!(config.context_aware);
    }

    #[test]
    fn test_completer_fuzzy_match() {
        let config = EnhancedInteractiveConfig::default();
        let completer = RhemaCompleter::new(&config);
        
        assert!(completer.fuzzy_match("init", "init"));
        assert!(completer.fuzzy_match("init", "i"));
        assert!(completer.fuzzy_match("init", "it"));
        assert!(!completer.fuzzy_match("init", "xyz"));
    }

    #[test]
    fn test_validator_balanced() {
        let validator = RhemaValidator::new();
        
        assert!(validator.is_balanced("echo 'hello world'"));
        assert!(validator.is_balanced("echo \"hello world\""));
        assert!(!validator.is_balanced("echo 'hello world"));
        assert!(!validator.is_balanced("echo (hello world"));
    }
} 