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

use crate::{Rhema, RhemaResult, RhemaError};
use colored::*;
use std::io::{self, Write};

/// Interactive command builder for complex operations
pub struct InteractiveBuilder {
    _rhema: Rhema,
}

impl InteractiveBuilder {
    pub fn new(rhema: Rhema) -> Self {
        Self { _rhema: rhema }
    }

    /// Build a todo add command interactively
    pub fn build_todo_add(&self) -> RhemaResult<String> {
        println!("{}", "📝 Interactive Todo Builder".bold().cyan());
        println!("{}", "=".repeat(40));
        
        // Get title
        let title = self.prompt_input("Todo title")?;
        
        // Get description
        let description = self.prompt_input_optional("Description (optional)")?;
        
        // Get priority
        let priority = self.prompt_priority()?;
        
        // Get assignee
        let assignee = self.prompt_input_optional("Assignee (optional)")?;
        
        // Get due date
        let due_date = self.prompt_input_optional("Due date (YYYY-MM-DD, optional)")?;
        
        // Build command
        let mut command = format!("todo add \"{}\"", title);
        
        if let Some(desc) = description {
            command.push_str(&format!(" --description \"{}\"", desc));
        }
        
        command.push_str(&format!(" --priority {}", priority));
        
        if let Some(assignee) = assignee {
            command.push_str(&format!(" --assignee \"{}\"", assignee));
        }
        
        if let Some(due_date) = due_date {
            command.push_str(&format!(" --due-date \"{}\"", due_date));
        }
        
        println!("\n{}", "Generated command:".bold().green());
        println!("{}", command.cyan());
        
        Ok(command)
    }

    /// Build an insight record command interactively
    pub fn build_insight_record(&self) -> RhemaResult<String> {
        println!("{}", "💡 Interactive Insight Builder".bold().cyan());
        println!("{}", "=".repeat(40));
        
        // Get title
        let title = self.prompt_input("Insight title")?;
        
        // Get content
        let content = self.prompt_input("Insight content")?;
        
        // Get confidence
        let confidence = self.prompt_confidence()?;
        
        // Get category
        let category = self.prompt_input_optional("Category (optional)")?;
        
        // Get tags
        let tags = self.prompt_input_optional("Tags (comma-separated, optional)")?;
        
        // Build command
        let mut command = format!("insight record \"{}\" --content \"{}\" --confidence {}", title, content, confidence);
        
        if let Some(category) = category {
            command.push_str(&format!(" --category \"{}\"", category));
        }
        
        if let Some(tags) = tags {
            command.push_str(&format!(" --tags \"{}\"", tags));
        }
        
        println!("\n{}", "Generated command:".bold().green());
        println!("{}", command.cyan());
        
        Ok(command)
    }

    /// Build a pattern add command interactively
    pub fn build_pattern_add(&self) -> RhemaResult<String> {
        println!("{}", "🔧 Interactive Pattern Builder".bold().cyan());
        println!("{}", "=".repeat(40));
        
        // Get name
        let name = self.prompt_input("Pattern name")?;
        
        // Get description
        let description = self.prompt_input_optional("Description (optional)")?;
        
        // Get pattern type
        let pattern_type = self.prompt_input_optional("Pattern type (optional)")?;
        
        // Get usage
        let usage = self.prompt_usage()?;
        
        // Get effectiveness
        let effectiveness = self.prompt_effectiveness()?;
        
        // Build command
        let mut command = format!("pattern add \"{}\" --usage {} --effectiveness {}", name, usage, effectiveness);
        
        if let Some(desc) = description {
            command.push_str(&format!(" --description \"{}\"", desc));
        }
        
        if let Some(pattern_type) = pattern_type {
            command.push_str(&format!(" --type \"{}\"", pattern_type));
        }
        
        println!("\n{}", "Generated command:".bold().green());
        println!("{}", command.cyan());
        
        Ok(command)
    }

    /// Build a decision record command interactively
    pub fn build_decision_record(&self) -> RhemaResult<String> {
        println!("{}", "🎯 Interactive Decision Builder".bold().cyan());
        println!("{}", "=".repeat(40));
        
        // Get title
        let title = self.prompt_input("Decision title")?;
        
        // Get description
        let description = self.prompt_input_optional("Description (optional)")?;
        
        // Get status
        let status = self.prompt_status()?;
        
        // Get maker
        let maker = self.prompt_input_optional("Decision maker (optional)")?;
        
        // Get rationale
        let rationale = self.prompt_input_optional("Rationale (optional)")?;
        
        // Build command
        let mut command = format!("decision record \"{}\" --status {}", title, status);
        
        if let Some(desc) = description {
            command.push_str(&format!(" --description \"{}\"", desc));
        }
        
        if let Some(maker) = maker {
            command.push_str(&format!(" --maker \"{}\"", maker));
        }
        
        if let Some(rationale) = rationale {
            command.push_str(&format!(" --rationale \"{}\"", rationale));
        }
        
        println!("\n{}", "Generated command:".bold().green());
        println!("{}", command.cyan());
        
        Ok(command)
    }

    /// Build a query command interactively
    pub fn build_query(&self) -> RhemaResult<String> {
        println!("{}", "🔍 Interactive Query Builder".bold().cyan());
        println!("{}", "=".repeat(40));
        
        // Show available query examples
        println!("{}", "Available query examples:".yellow());
        println!("  1. SELECT * FROM scopes");
        println!("  2. SELECT name, description FROM scopes");
        println!("  3. SELECT * FROM todos WHERE status='in_progress'");
        println!("  4. SELECT * FROM insights WHERE category='architecture'");
        println!("  5. Custom query");
        println!();
        
        let choice = self.prompt_choice("Choose query type (1-5)", &["1", "2", "3", "4", "5"])?;
        
        let query = match choice.as_str() {
            "1" => "SELECT * FROM scopes".to_string(),
            "2" => "SELECT name, description FROM scopes".to_string(),
            "3" => "SELECT * FROM todos WHERE status='in_progress'".to_string(),
            "4" => "SELECT * FROM insights WHERE category='architecture'".to_string(),
            "5" => self.prompt_input("Enter custom query")?,
            _ => return Err(RhemaError::InvalidCommand("Invalid choice".to_string())),
        };
        
        // Ask for additional options
        let mut command = format!("query \"{}\"", query);
        
        if self.prompt_yes_no("Include statistics")? {
            command.push_str(" --stats");
        }
        
        let format = self.prompt_format()?;
        if format != "yaml" {
            command.push_str(&format!(" --format {}", format));
        }
        
        println!("\n{}", "Generated command:".bold().green());
        println!("{}", command.cyan());
        
        Ok(command)
    }

    /// Build a prompt pattern interactively
    pub fn build_prompt_pattern(&self) -> RhemaResult<String> {
        println!("{}", "🎯 Interactive Prompt Pattern Builder".bold().cyan());
        println!("{}", "=".repeat(50));
        
        let name = self.prompt_input("Prompt pattern name")?;
        let description = self.prompt_input_optional("Description (optional)")?;
        let template = self.prompt_template()?;
        let injection = self.prompt_injection_method()?;
        let tags = self.prompt_input_optional("Tags (comma-separated, optional)")?;
        
        let mut command = format!("prompt add \"{}\" --template \"{}\"", name, template);
        
        if let Some(desc) = description {
            command.push_str(&format!(" --description \"{}\"", desc));
        }
        command.push_str(&format!(" --injection {}", injection));
        if let Some(tags_str) = tags {
            command.push_str(&format!(" --tags \"{}\"", tags_str));
        }
        
        println!("\n{}", "Generated command:".bold().green());
        println!("{}", command.cyan());
        
        Ok(command)
    }
    
    fn prompt_template(&self) -> RhemaResult<String> {
        println!("\n{}", "Enter your prompt template:".yellow());
        println!("Use {{CONTEXT}} for context injection placeholder");
        println!("Press Enter twice to finish:");
        
        let mut template = String::new();
        let mut empty_lines = 0;
        
        loop {
            let line = self.prompt_input_optional("Template line")?;
            match line {
                Some(l) => {
                    template.push_str(&l);
                    template.push('\n');
                    empty_lines = 0;
                }
                None => {
                    empty_lines += 1;
                    if empty_lines >= 2 {
                        break;
                    }
                }
            }
        }
        
        Ok(template.trim().to_string())
    }
    
    fn prompt_injection_method(&self) -> RhemaResult<String> {
        println!("\n{}", "Choose context injection method:".yellow());
        println!("1. prepend - Add context before the prompt");
        println!("2. append - Add context after the prompt");
        println!("3. template_variable - Use {{CONTEXT}} placeholder");
        
        let choice = self.prompt_choice("Choose injection method (1-3)", &["1", "2", "3"])?;
        
        match choice.as_str() {
            "1" => Ok("prepend".to_string()),
            "2" => Ok("append".to_string()),
            "3" => Ok("template_variable".to_string()),
            _ => Ok("template_variable".to_string()), // Default
        }
    }

    /// Prompt for user input
    fn prompt_input(&self, prompt: &str) -> RhemaResult<String> {
        print!("{}: ", prompt.cyan());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim().to_string();
        if input.is_empty() {
            return Err(RhemaError::InvalidCommand(format!("{} cannot be empty", prompt)));
        }
        
        Ok(input)
    }

    /// Prompt for optional user input
    fn prompt_input_optional(&self, prompt: &str) -> RhemaResult<Option<String>> {
        print!("{}: ", prompt.cyan());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim().to_string();
        if input.is_empty() {
            Ok(None)
        } else {
            Ok(Some(input))
        }
    }

    /// Prompt for priority selection
    fn prompt_priority(&self) -> RhemaResult<String> {
        println!("{}", "Priority levels:".yellow());
        println!("  1. Low");
        println!("  2. Medium");
        println!("  3. High");
        println!("  4. Critical");
        println!();
        
        let choice = self.prompt_choice("Choose priority (1-4)", &["1", "2", "3", "4"])?;
        
        Ok(match choice.as_str() {
            "1" => "low".to_string(),
            "2" => "medium".to_string(),
            "3" => "high".to_string(),
            "4" => "critical".to_string(),
            _ => return Err(RhemaError::InvalidCommand("Invalid priority".to_string())),
        })
    }

    /// Prompt for confidence level
    fn prompt_confidence(&self) -> RhemaResult<u8> {
        println!("{}", "Confidence level (1-10):".yellow());
        println!("  1-3: Low confidence");
        println!("  4-6: Medium confidence");
        println!("  7-9: High confidence");
        println!("  10: Very high confidence");
        println!();
        
        let choice = self.prompt_choice("Choose confidence (1-10)", &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"])?;
        
        choice.parse::<u8>()
            .map_err(|_| RhemaError::InvalidCommand("Invalid confidence level".to_string()))
    }

    /// Prompt for usage selection
    fn prompt_usage(&self) -> RhemaResult<String> {
        println!("{}", "Usage types:".yellow());
        println!("  1. Recommended");
        println!("  2. Optional");
        println!("  3. Deprecated");
        println!();
        
        let choice = self.prompt_choice("Choose usage (1-3)", &["1", "2", "3"])?;
        
        Ok(match choice.as_str() {
            "1" => "recommended".to_string(),
            "2" => "optional".to_string(),
            "3" => "deprecated".to_string(),
            _ => return Err(RhemaError::InvalidCommand("Invalid usage".to_string())),
        })
    }

    /// Prompt for effectiveness level
    fn prompt_effectiveness(&self) -> RhemaResult<u8> {
        println!("{}", "Effectiveness level (1-10):".yellow());
        println!("  1-3: Low effectiveness");
        println!("  4-6: Medium effectiveness");
        println!("  7-9: High effectiveness");
        println!("  10: Very high effectiveness");
        println!();
        
        let choice = self.prompt_choice("Choose effectiveness (1-10)", &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"])?;
        
        choice.parse::<u8>()
            .map_err(|_| RhemaError::InvalidCommand("Invalid effectiveness level".to_string()))
    }

    /// Prompt for status selection
    fn prompt_status(&self) -> RhemaResult<String> {
        println!("{}", "Status types:".yellow());
        println!("  1. Proposed");
        println!("  2. Approved");
        println!("  3. Rejected");
        println!("  4. Superseded");
        println!();
        
        let choice = self.prompt_choice("Choose status (1-4)", &["1", "2", "3", "4"])?;
        
        Ok(match choice.as_str() {
            "1" => "proposed".to_string(),
            "2" => "approved".to_string(),
            "3" => "rejected".to_string(),
            "4" => "superseded".to_string(),
            _ => return Err(RhemaError::InvalidCommand("Invalid status".to_string())),
        })
    }

    /// Prompt for format selection
    fn prompt_format(&self) -> RhemaResult<String> {
        println!("{}", "Output formats:".yellow());
        println!("  1. YAML (default)");
        println!("  2. JSON");
        println!("  3. Table");
        println!("  4. Count");
        println!();
        
        let choice = self.prompt_choice("Choose format (1-4)", &["1", "2", "3", "4"])?;
        
        Ok(match choice.as_str() {
            "1" => "yaml".to_string(),
            "2" => "json".to_string(),
            "3" => "table".to_string(),
            "4" => "count".to_string(),
            _ => return Err(RhemaError::InvalidCommand("Invalid format".to_string())),
        })
    }

    /// Prompt for choice from a list
    fn prompt_choice(&self, prompt: &str, choices: &[&str]) -> RhemaResult<String> {
        loop {
            print!("{}: ", prompt.cyan());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            let input = input.trim().to_string();
            if choices.contains(&input.as_str()) {
                return Ok(input);
            } else {
                println!("{}", "Invalid choice. Please try again.".red());
            }
        }
    }

    /// Prompt for yes/no answer
    fn prompt_yes_no(&self, prompt: &str) -> RhemaResult<bool> {
        loop {
            print!("{} (y/n): ", prompt.cyan());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let rhema = Rhema::new().unwrap();
        let builder = InteractiveBuilder::new(rhema);
        assert!(true); // Just test that it can be created
    }
} 