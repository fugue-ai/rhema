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

use super::*;
use rhema_action_tool::{ActionIntent, ActionType, SafetyLevel};
use serde_json::json;
use std::os::unix::process::ExitStatusExt;

#[tokio::test]
async fn test_cargo_tool_creation() {
    let tool = CargoTool;
    assert_eq!(rhema_action_tool::ValidationTool::name(&tool), "cargo");
    assert_eq!(rhema_action_tool::ValidationTool::version(&tool), "1.0.0");
}

#[tokio::test]
async fn test_parse_config_default() {
    let tool = CargoTool;
    let intent = ActionIntent::new(
        "test",
        ActionType::Test,
        "Test validation",
        vec![],
        SafetyLevel::Low,
    );

    let config = tool.parse_config(&intent);
    assert_eq!(config.commands, vec![CargoCommand::Check]);
    assert!(config.parallel);
    assert!(config.json_output);
    assert!(!config.verbose);
    assert_eq!(config.workspace_mode, WorkspaceMode::RootAndMembers);
    assert!(config.member_filter.is_none());
    assert!(config.exclude_members.is_none());
}

#[tokio::test]
async fn test_parse_config_custom() {
    let tool = CargoTool;
    let mut intent = ActionIntent::new(
        "test",
        ActionType::Test,
        "Test validation",
        vec![],
        SafetyLevel::Low,
    );
    intent.metadata = json!({
        "commands": ["check", "clippy", "test"],
        "parallel": false,
        "json_output": false,
        "verbose": true,
        "workspace_mode": "all_members",
        "member_filter": ["core", "api"],
        "exclude_members": ["tests"]
    });

    let config = tool.parse_config(&intent);
    assert_eq!(
        config.commands,
        vec![
            CargoCommand::Check,
            CargoCommand::Clippy,
            CargoCommand::Test
        ]
    );
    assert!(!config.parallel);
    assert!(!config.json_output);
    assert!(config.verbose);
    assert_eq!(config.workspace_mode, WorkspaceMode::AllMembers);
    assert_eq!(
        config.member_filter,
        Some(vec!["core".to_string(), "api".to_string()])
    );
    assert_eq!(config.exclude_members, Some(vec!["tests".to_string()]));
}

#[tokio::test]
async fn test_parse_config_workspace_modes() {
    let tool = CargoTool;

    // Test root_only mode
    let mut intent = ActionIntent::new(
        "test",
        ActionType::Test,
        "Test validation",
        vec![],
        SafetyLevel::Low,
    );
    intent.metadata = json!({
        "workspace_mode": "root_only"
    });
    let config = tool.parse_config(&intent);
    assert_eq!(config.workspace_mode, WorkspaceMode::RootOnly);

    // Test all_members mode
    intent.metadata = json!({
        "workspace_mode": "all_members"
    });
    let config = tool.parse_config(&intent);
    assert_eq!(config.workspace_mode, WorkspaceMode::AllMembers);

    // Test root_and_members mode
    intent.metadata = json!({
        "workspace_mode": "root_and_members"
    });
    let config = tool.parse_config(&intent);
    assert_eq!(config.workspace_mode, WorkspaceMode::RootAndMembers);

    // Test selected_members mode
    intent.metadata = json!({
        "workspace_mode": "selected_members"
    });
    let config = tool.parse_config(&intent);
    assert_eq!(config.workspace_mode, WorkspaceMode::SelectedMembers);

    // Test invalid mode (should default to root_and_members)
    intent.metadata = json!({
        "workspace_mode": "invalid_mode"
    });
    let config = tool.parse_config(&intent);
    assert_eq!(config.workspace_mode, WorkspaceMode::RootAndMembers);
}

#[tokio::test]
async fn test_build_command_args() {
    let tool = CargoTool;
    let config = CargoConfig {
        commands: vec![],
        parallel: true,
        json_output: true,
        verbose: false,
        workspace_mode: WorkspaceMode::RootAndMembers,
        member_filter: None,
        exclude_members: None,
    };

    // Test check command
    let (cmd, args) = tool.build_command_args(&CargoCommand::Check, &config);
    assert_eq!(cmd, "cargo");
    assert!(args.contains(&"check"));
    assert!(args.contains(&"--message-format=json"));

    // Test build command
    let (cmd, args) = tool.build_command_args(&CargoCommand::Build, &config);
    assert_eq!(cmd, "cargo");
    assert!(args.contains(&"build"));
    assert!(args.contains(&"--message-format=json"));

    // Test test command
    let (cmd, args) = tool.build_command_args(&CargoCommand::Test, &config);
    assert_eq!(cmd, "cargo");
    assert!(args.contains(&"test"));
    assert!(args.contains(&"--message-format=json"));

    // Test clippy command
    let (cmd, args) = tool.build_command_args(&CargoCommand::Clippy, &config);
    assert_eq!(cmd, "cargo");
    assert!(args.contains(&"clippy"));
    assert!(args.contains(&"--message-format=json"));

    // Test fmt command
    let (cmd, args) = tool.build_command_args(&CargoCommand::Fmt, &config);
    assert_eq!(cmd, "cargo");
    assert!(args.contains(&"fmt"));
    assert!(args.contains(&"--message-format=json"));

    // Test audit command
    let (cmd, args) = tool.build_command_args(&CargoCommand::Audit, &config);
    assert_eq!(cmd, "cargo");
    assert!(args.contains(&"audit"));
    assert!(args.contains(&"--output-format=json"));

    // Test outdated command
    let (cmd, args) = tool.build_command_args(&CargoCommand::Outdated, &config);
    assert_eq!(cmd, "cargo");
    assert!(args.contains(&"outdated"));
    assert!(args.contains(&"--format=json"));
}

#[tokio::test]
async fn test_build_command_args_verbose() {
    let tool = CargoTool;
    let config = CargoConfig {
        commands: vec![],
        parallel: true,
        json_output: true,
        verbose: true,
        workspace_mode: WorkspaceMode::RootAndMembers,
        member_filter: None,
        exclude_members: None,
    };

    let (cmd, args) = tool.build_command_args(&CargoCommand::Check, &config);
    assert_eq!(cmd, "cargo");
    assert!(args.contains(&"check"));
    assert!(args.contains(&"--message-format=json"));
    assert!(args.contains(&"--verbose"));
}

#[tokio::test]
async fn test_parse_cargo_output_json() {
    let tool = CargoTool;
    let config = CargoConfig {
        commands: vec![],
        parallel: true,
        json_output: true,
        verbose: false,
        workspace_mode: WorkspaceMode::RootAndMembers,
        member_filter: None,
        exclude_members: None,
    };

    // Mock JSON output
    let json_output = r#"{"message":{"level":"error","message":"expected `;`, found `}`","spans":[{"file_name":"src/main.rs","line_start":15}]}}
{"message":{"level":"warning","message":"unused variable: `x`","spans":[{"file_name":"src/lib.rs","line_start":10}]}}"#;

    let output = std::process::Output {
        status: std::process::ExitStatus::from_raw(1),
        stdout: json_output.as_bytes().to_vec(),
        stderr: vec![],
    };

    let (errors, warnings) = tool.parse_cargo_output(&output, &CargoCommand::Check, &config);

    assert_eq!(errors.len(), 1);
    assert_eq!(warnings.len(), 1);
    assert!(errors[0].contains("src/main.rs:15"));
    assert!(warnings[0].contains("src/lib.rs:10"));
}

#[tokio::test]
async fn test_parse_cargo_output_stderr() {
    let tool = CargoTool;
    let config = CargoConfig {
        commands: vec![],
        parallel: true,
        json_output: false,
        verbose: false,
        workspace_mode: WorkspaceMode::RootAndMembers,
        member_filter: None,
        exclude_members: None,
    };

    let output = std::process::Output {
        status: std::process::ExitStatus::from_raw(1),
        stdout: vec![],
        stderr: b"error: expected `;`, found `}`\nwarning: unused variable: `x`".to_vec(),
    };

    let (errors, warnings) = tool.parse_cargo_output(&output, &CargoCommand::Check, &config);

    assert_eq!(errors.len(), 1);
    assert_eq!(warnings.len(), 1);
    assert!(errors[0].contains("error:"));
    assert!(warnings[0].contains("warning:"));
}

#[tokio::test]
async fn test_transformation_tool_traits() {
    let tool = CargoTool;

    // Test language support
    assert!(tool.supports_language("rust"));
    assert!(!tool.supports_language("javascript"));
    assert!(!tool.supports_language("python"));

    // Test safety level
    assert_eq!(tool.safety_level(), SafetyLevel::Medium);
}

#[tokio::test]
async fn test_cargo_command_enum() {
    // Test command equality
    assert_eq!(CargoCommand::Check, CargoCommand::Check);
    assert_ne!(CargoCommand::Check, CargoCommand::Build);

    // Test command cloning
    let cmd = CargoCommand::Clippy;
    let cloned = cmd.clone();
    assert_eq!(cmd, cloned);
}

#[tokio::test]
async fn test_cargo_config_default() {
    let config = CargoConfig::default();
    assert_eq!(config.commands, vec![CargoCommand::Check]);
    assert!(config.parallel);
    assert!(config.json_output);
    assert!(!config.verbose);
    assert_eq!(config.workspace_mode, WorkspaceMode::RootAndMembers);
    assert!(config.member_filter.is_none());
    assert!(config.exclude_members.is_none());
}

#[tokio::test]
async fn test_cargo_result_structure() {
    let result = CargoResult {
        command: CargoCommand::Check,
        success: true,
        output: "Success".to_string(),
        errors: vec![],
        warnings: vec!["Warning".to_string()],
        duration: std::time::Duration::from_secs(1),
    };

    assert_eq!(result.command, CargoCommand::Check);
    assert!(result.success);
    assert_eq!(result.output, "Success");
    assert!(result.errors.is_empty());
    assert_eq!(result.warnings.len(), 1);
    assert_eq!(result.duration, std::time::Duration::from_secs(1));
}

#[tokio::test]
async fn test_validation_with_no_cargo_files() {
    let tool = CargoTool;
    let intent = ActionIntent::new(
        "test",
        ActionType::Test,
        "Test validation",
        vec!["src/main.rs".to_string(), "README.md".to_string()],
        SafetyLevel::Low,
    );

    let result = tool.validate(&intent).await.unwrap();
    assert!(result.success);
    assert_eq!(result.output, "No Cargo.toml files found to validate");
    assert!(result.errors.is_empty());
    assert!(result.warnings.is_empty());
}

#[tokio::test]
async fn test_transformation_with_no_cargo_files() {
    let tool = CargoTool;
    let intent = ActionIntent::new(
        "test",
        ActionType::Refactor,
        "Test transformation",
        vec!["src/main.rs".to_string(), "README.md".to_string()],
        SafetyLevel::Medium,
    );

    let result = tool.execute(&intent).await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("No Cargo.toml files found"));
}

#[tokio::test]
async fn test_workspace_mode_enum() {
    // Test workspace mode equality
    assert_eq!(WorkspaceMode::RootOnly, WorkspaceMode::RootOnly);
    assert_ne!(WorkspaceMode::RootOnly, WorkspaceMode::AllMembers);

    // Test workspace mode cloning
    let mode = WorkspaceMode::SelectedMembers;
    let cloned = mode.clone();
    assert_eq!(mode, cloned);
}

#[tokio::test]
async fn test_package_type_enum() {
    // Test package type equality
    assert_eq!(PackageType::Library, PackageType::Library);
    assert_ne!(PackageType::Library, PackageType::Binary);

    // Test package type cloning
    let pkg_type = PackageType::Both;
    let cloned = pkg_type.clone();
    assert_eq!(pkg_type, cloned);
}

#[tokio::test]
async fn test_workspace_member_structure() {
    let member = WorkspaceMember {
        name: "test-crate".to_string(),
        path: "crates/test-crate".to_string(),
        package_type: PackageType::Library,
    };

    assert_eq!(member.name, "test-crate");
    assert_eq!(member.path, "crates/test-crate");
    assert_eq!(member.package_type, PackageType::Library);
}

#[tokio::test]
async fn test_workspace_info_structure() {
    let workspace_info = WorkspaceInfo {
        root_path: "/path/to/workspace".to_string(),
        members: vec![
            WorkspaceMember {
                name: "core".to_string(),
                path: "crates/core".to_string(),
                package_type: PackageType::Library,
            },
            WorkspaceMember {
                name: "api".to_string(),
                path: "crates/api".to_string(),
                package_type: PackageType::Binary,
            },
        ],
        workspace_config: Some(json!({
            "resolver": "2"
        })),
    };

    assert_eq!(workspace_info.root_path, "/path/to/workspace");
    assert_eq!(workspace_info.members.len(), 2);
    assert_eq!(workspace_info.members[0].name, "core");
    assert_eq!(workspace_info.members[1].name, "api");
    assert!(workspace_info.workspace_config.is_some());
}

#[tokio::test]
async fn test_get_selected_members() {
    let tool = CargoTool;
    let members = vec![
        WorkspaceMember {
            name: "core".to_string(),
            path: "crates/core".to_string(),
            package_type: PackageType::Library,
        },
        WorkspaceMember {
            name: "api".to_string(),
            path: "crates/api".to_string(),
            package_type: PackageType::Binary,
        },
        WorkspaceMember {
            name: "tests".to_string(),
            path: "crates/tests".to_string(),
            package_type: PackageType::Library,
        },
    ];

    // Test with no filters
    let config = CargoConfig {
        commands: vec![],
        parallel: true,
        json_output: true,
        verbose: false,
        workspace_mode: WorkspaceMode::SelectedMembers,
        member_filter: None,
        exclude_members: None,
    };

    let selected = tool.get_selected_members(&members, &config);
    assert_eq!(selected.len(), 3);

    // Test with member filter
    let config = CargoConfig {
        commands: vec![],
        parallel: true,
        json_output: true,
        verbose: false,
        workspace_mode: WorkspaceMode::SelectedMembers,
        member_filter: Some(vec!["core".to_string(), "api".to_string()]),
        exclude_members: None,
    };

    let selected = tool.get_selected_members(&members, &config);
    assert_eq!(selected.len(), 2);
    assert_eq!(selected[0].name, "core");
    assert_eq!(selected[1].name, "api");

    // Test with exclude filter
    let config = CargoConfig {
        commands: vec![],
        parallel: true,
        json_output: true,
        verbose: false,
        workspace_mode: WorkspaceMode::SelectedMembers,
        member_filter: None,
        exclude_members: Some(vec!["tests".to_string()]),
    };

    let selected = tool.get_selected_members(&members, &config);
    assert_eq!(selected.len(), 2);
    assert_eq!(selected[0].name, "core");
    assert_eq!(selected[1].name, "api");

    // Test with both filters
    let config = CargoConfig {
        commands: vec![],
        parallel: true,
        json_output: true,
        verbose: false,
        workspace_mode: WorkspaceMode::SelectedMembers,
        member_filter: Some(vec![
            "core".to_string(),
            "api".to_string(),
            "tests".to_string(),
        ]),
        exclude_members: Some(vec!["tests".to_string()]),
    };

    let selected = tool.get_selected_members(&members, &config);
    assert_eq!(selected.len(), 2);
    assert_eq!(selected[0].name, "core");
    assert_eq!(selected[1].name, "api");
}

#[tokio::test]
async fn test_extract_workspace_members() {
    let tool = CargoTool;

    // Test with valid workspace members
    let cargo_content = r#"
[package]
name = "workspace-root"
version = "0.1.0"

[workspace]
members = [
    "crates/core",
    "crates/api",
    "crates/tests"
]
"#;

    let members = tool.extract_workspace_members(cargo_content);
    assert!(members.is_some());
    let members = members.unwrap();
    assert_eq!(members.len(), 3);
    assert_eq!(members[0], "crates/core");
    assert_eq!(members[1], "crates/api");
    assert_eq!(members[2], "crates/tests");

    // Test with no workspace section
    let cargo_content = r#"
[package]
name = "single-crate"
version = "0.1.0"
"#;

    let members = tool.extract_workspace_members(cargo_content);
    assert!(members.is_none());

    // Test with empty members
    let cargo_content = r#"
[package]
name = "workspace-root"
version = "0.1.0"

[workspace]
members = []
"#;

    let members = tool.extract_workspace_members(cargo_content);
    assert!(members.is_none());
}

#[tokio::test]
async fn test_extract_workspace_config() {
    let tool = CargoTool;

    // Test with resolver configuration
    let cargo_content = r#"
[workspace]
resolver = "2"
members = ["crates/core"]
"#;

    let config = tool.extract_workspace_config(cargo_content);
    assert!(config.is_some());
    let config = config.unwrap();
    assert_eq!(config["resolver"], "2");

    // Test without resolver configuration
    let cargo_content = r#"
[workspace]
members = ["crates/core"]
"#;

    let config = tool.extract_workspace_config(cargo_content);
    assert!(config.is_none());
}
