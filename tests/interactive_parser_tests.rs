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

use rhema::commands::interactive_parser::InteractiveCommandParser;

#[test]
fn test_parse_input_basic() {
    let parser = InteractiveCommandParser::new("todo add \"Implement user auth\" --priority high");
    assert_eq!(
        parser.parts,
        vec!["todo", "add", "Implement user auth", "--priority", "high"]
    );
}

#[test]
fn test_parse_input_with_quotes() {
    let parser = InteractiveCommandParser::new(
        "todo add \"Implement user authentication\" --description \"Add OAuth2 support\"",
    );
    assert_eq!(
        parser.parts,
        vec![
            "todo",
            "add",
            "Implement user authentication",
            "--description",
            "Add OAuth2 support"
        ]
    );
}

#[test]
fn test_parse_input_with_escaped_chars() {
    let parser = InteractiveCommandParser::new("todo add \"Fix \\\"quoted\\\" text\"");
    assert_eq!(parser.parts, vec!["todo", "add", "Fix \"quoted\" text"]);
}

#[test]
fn test_command_extraction() {
    let parser = InteractiveCommandParser::new("todo add \"Test todo\" --priority high");
    assert_eq!(parser.command(), Some("todo"));
}

#[test]
fn test_args_extraction() {
    let parser = InteractiveCommandParser::new("todo add \"Test todo\" --priority high");
    assert_eq!(
        parser.args(),
        vec!["add", "Test todo", "--priority", "high"]
    );
}

#[test]
fn test_next_and_peek() {
    let mut parser = InteractiveCommandParser::new("todo add \"Test todo\" --priority high");

    // Skip the command
    parser.next();

    assert_eq!(parser.peek(), Some("add"));
    assert_eq!(parser.next(), Some("add"));
    assert_eq!(parser.next(), Some("Test todo"));
    assert_eq!(parser.next(), Some("--priority"));
    assert_eq!(parser.next(), Some("high"));
    assert_eq!(parser.next(), None);
}

#[test]
fn test_remaining() {
    let mut parser = InteractiveCommandParser::new("todo add \"Test todo\" --priority high");

    // Skip the command
    parser.next();

    let remaining: Vec<String> = parser.remaining().iter().map(|s| s.to_string()).collect();
    assert_eq!(remaining, vec!["add", "Test todo", "--priority", "high"]);
}

#[test]
fn test_has_more() {
    let mut parser = InteractiveCommandParser::new("todo add \"Test todo\"");

    assert!(parser.has_more());
    parser.next(); // Skip command
    assert!(parser.has_more());
    parser.next(); // Skip subcommand
    assert!(parser.has_more());
    parser.next(); // Skip title
    assert!(!parser.has_more());
}

#[test]
fn test_reset() {
    let mut parser = InteractiveCommandParser::new("todo add \"Test todo\" --priority high");

    parser.next(); // Skip command
    parser.next(); // Skip subcommand
    assert_eq!(parser.next(), Some("Test todo"));

    parser.reset();
    assert_eq!(parser.command(), Some("todo"));
    assert_eq!(parser.next(), Some("todo"));
}
