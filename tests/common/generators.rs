//! Test data generators for property-based testing

use proptest::prelude::*;
use serde_yaml::Value;
use std::collections::HashMap;

/// Generators for Rhema test data
pub struct TestGenerators;

impl TestGenerators {
    /// Generate random todo data
    pub fn todo_generator() -> impl Strategy<Value = HashMap<String, Value>> {
        (
            prop::sample::select(vec!["todo-001", "todo-002", "todo-003", "todo-004", "todo-005"]),
            prop::sample::select(vec!["Test todo", "Important task", "Bug fix", "Feature implementation", "Documentation update"]),
            prop::sample::select(vec!["pending", "in_progress", "completed", "cancelled"]),
            prop::sample::select(vec!["low", "medium", "high", "critical"]),
            prop::sample::select(vec!["developer1", "developer2", "developer3", "tester", "reviewer"]),
        )
        .prop_map(|(id, title, status, priority, assignee)| {
            let mut todo = HashMap::new();
            todo.insert("id".to_string(), Value::String(id.to_string()));
            todo.insert("title".to_string(), Value::String(title.to_string()));
            todo.insert("status".to_string(), Value::String(status.to_string()));
            todo.insert("priority".to_string(), Value::String(priority.to_string()));
            todo.insert("assignee".to_string(), Value::String(assignee.to_string()));
            todo.insert("created_at".to_string(), Value::String("2024-01-15T10:00:00Z".to_string()));
            todo
        })
    }
    
    /// Generate random insight data
    pub fn insight_generator() -> impl Strategy<Value = HashMap<String, Value>> {
        (
            prop::sample::select(vec!["insight-001", "insight-002", "insight-003", "insight-004", "insight-005"]),
            prop::sample::select(vec!["Performance optimization", "Security finding", "Architecture insight", "Code quality observation", "User experience insight"]),
            prop::sample::select(vec!["This is a valuable insight about system performance.", "Security vulnerability discovered in authentication flow.", "Architecture pattern that improves maintainability.", "Code quality issue that affects readability.", "User experience improvement opportunity identified."]),
            (1..=10u8),
            prop::sample::select(vec!["performance", "security", "architecture", "code-quality", "user-experience"]),
        )
        .prop_map(|(id, title, content, confidence, category)| {
            let mut insight = HashMap::new();
            insight.insert("id".to_string(), Value::String(id.to_string()));
            insight.insert("title".to_string(), Value::String(title.to_string()));
            insight.insert("content".to_string(), Value::String(content.to_string()));
            insight.insert("confidence".to_string(), Value::Number(confidence.into()));
            insight.insert("category".to_string(), Value::String(category.to_string()));
            insight.insert("created_at".to_string(), Value::String("2024-01-15T10:00:00Z".to_string()));
            insight
        })
    }
    
    /// Generate random pattern data
    pub fn pattern_generator() -> impl Strategy<Value = HashMap<String, Value>> {
        (
            prop::sample::select(vec!["pattern-001", "pattern-002", "pattern-003", "pattern-004", "pattern-005"]),
            prop::sample::select(vec!["Repository Pattern", "Factory Pattern", "Observer Pattern", "Strategy Pattern", "Command Pattern"]),
            prop::sample::select(vec!["Centralize data access logic", "Create objects without specifying exact classes", "Define a one-to-many dependency between objects", "Define a family of algorithms", "Encapsulate a request as an object"]),
            prop::sample::select(vec!["architectural", "creational", "behavioral", "structural", "concurrency"]),
            prop::sample::select(vec!["recommended", "optional", "deprecated"]),
            (1..=10u8),
        )
        .prop_map(|(id, name, description, pattern_type, usage, effectiveness)| {
            let mut pattern = HashMap::new();
            pattern.insert("id".to_string(), Value::String(id.to_string()));
            pattern.insert("name".to_string(), Value::String(name.to_string()));
            pattern.insert("description".to_string(), Value::String(description.to_string()));
            pattern.insert("pattern_type".to_string(), Value::String(pattern_type.to_string()));
            pattern.insert("usage".to_string(), Value::String(usage.to_string()));
            pattern.insert("effectiveness".to_string(), Value::Number(effectiveness.into()));
            pattern.insert("created_at".to_string(), Value::String("2024-01-15T10:00:00Z".to_string()));
            pattern
        })
    }
    
    /// Generate random decision data
    pub fn decision_generator() -> impl Strategy<Value = HashMap<String, Value>> {
        (
            prop::sample::select(vec!["decision-001", "decision-002", "decision-003", "decision-004", "decision-005"]),
            prop::sample::select(vec!["Technology choice", "Architecture decision", "Process improvement", "Security measure", "Performance optimization"]),
            prop::sample::select(vec!["Decision about technology stack selection", "Architecture pattern implementation decision", "Process improvement initiative decision", "Security measure implementation decision", "Performance optimization strategy decision"]),
            prop::sample::select(vec!["proposed", "approved", "rejected", "implemented", "reviewed"]),
        )
        .prop_map(|(id, title, description, status)| {
            let mut decision = HashMap::new();
            decision.insert("id".to_string(), Value::String(id.to_string()));
            decision.insert("title".to_string(), Value::String(title.to_string()));
            decision.insert("description".to_string(), Value::String(description.to_string()));
            decision.insert("status".to_string(), Value::String(status.to_string()));
            decision.insert("created_at".to_string(), Value::String("2024-01-15T10:00:00Z".to_string()));
            decision
        })
    }
    
    /// Generate random scope data
    pub fn scope_generator() -> impl Strategy<Value = HashMap<String, Value>> {
        (
            prop::sample::select(vec!["test-scope", "service-scope", "library-scope", "api-scope", "cli-scope"]),
            prop::sample::select(vec!["service", "library", "api", "cli", "tool"]),
            prop::sample::select(vec!["Test scope for unit testing", "Service scope for backend services", "Library scope for shared components", "API scope for external interfaces", "CLI scope for command line tools"]),
            prop::sample::select(vec!["1.0.0", "2.0.0", "3.0.0", "1.1.0", "2.1.0"]),
        )
        .prop_map(|(name, scope_type, description, version)| {
            let mut scope = HashMap::new();
            scope.insert("name".to_string(), Value::String(name.to_string()));
            scope.insert("scope_type".to_string(), Value::String(scope_type.to_string()));
            scope.insert("description".to_string(), Value::String(description.to_string()));
            scope.insert("version".to_string(), Value::String(version.to_string()));
            scope.insert("schema_version".to_string(), Value::String("1.0.0".to_string()));
            scope.insert("dependencies".to_string(), Value::Null);
            scope
        })
    }
    
    /// Generate random query strings
    pub fn query_generator() -> impl Strategy<Value = String> {
        (
            prop::sample::select(vec!["todos", "insights", "patterns", "decisions"]),
            prop::sample::select(vec![
                None,
                Some("WHERE status=pending"),
                Some("WHERE priority=high"),
                Some("WHERE assignee=developer1"),
                Some("WHERE status=pending AND priority=high"),
                Some("WHERE confidence>=8"),
                Some("WHERE pattern_type=architectural"),
                Some("WHERE status=approved"),
            ]),
        )
        .prop_map(|(target, condition)| {
            if let Some(cond) = condition {
                format!("{} {}", target, cond)
            } else {
                target.to_string()
            }
        })
    }
    
    /// Generate random file paths
    pub fn file_path_generator() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            ".rhema/todos.yaml",
            ".rhema/insights.yaml",
            ".rhema/patterns.yaml",
            ".rhema/decisions.yaml",
            ".rhema/schemas.yaml",
            ".rhema/data/todos.yaml",
            ".rhema/data/insights.yaml",
            ".rhema/schemas/todo.yaml",
            ".rhema/schemas/insight.yaml",
        ]).prop_map(|s| s.to_string())
    }
    
    /// Generate random YAML content
    pub fn yaml_content_generator() -> impl Strategy<Value = String> {
        (
            prop::sample::select(vec!["todos", "insights", "patterns", "decisions"]),
            (1..=5usize),
        )
        .prop_map(|(data_type, count)| {
            let mut content = format!("{}:\n", data_type);
            for i in 0..count {
                content.push_str(&format!("  - id: \"{}-{:03}\"\n", data_type, i + 1));
                content.push_str(&format!("    title: \"Generated {} {}\"\n", data_type, i + 1));
                content.push_str(&format!("    created_at: \"2024-01-{:02}T10:00:00Z\"\n", (i % 30) + 1));
            }
            content
        })
    }
    
    /// Generate random error scenarios
    pub fn error_scenario_generator() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "invalid: yaml: content: [",
            "todos WHERE invalid_field=value",
            "nonexistent.yaml",
            "circular: dependency: reference",
            "/root/protected/file.yaml",
        ]).prop_map(|s| s.to_string())
    }
    
    /// Generate random performance test data sizes
    pub fn performance_size_generator() -> impl Strategy<Value = usize> {
        prop::sample::select(vec![100, 1000, 10000, 100000])
    }
    
    /// Generate random security test data
    pub fn security_test_generator() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "!!python/object/apply:os.system ['rm -rf /']",
            "!!python/object/apply:subprocess.check_output [['cat', '/etc/passwd']]",
            "../../../etc/passwd",
            "..\\..\\..\\windows\\system32\\config\\sam",
            "%2e%2e%2f%2e%2e%2f%2e%2e%2fetc%2fpasswd",
        ]).prop_map(|s| s.to_string())
    }
    
    /// Generate random file permissions
    pub fn file_permissions_generator() -> impl Strategy<Value = u32> {
        prop::sample::select(vec![0o644, 0o755, 0o600, 0o777, 0o400])
    }
    
    /// Generate random encoding types
    pub fn encoding_generator() -> impl Strategy<Value = String> {
        prop::sample::select(vec!["utf8", "utf16", "ascii"]).prop_map(|s| s.to_string())
    }
    
    /// Generate random line endings
    pub fn line_ending_generator() -> impl Strategy<Value = String> {
        prop::sample::select(vec!["\n", "\r\n", "\r"]).prop_map(|s| s.to_string())
    }
    
    /// Generate random file attributes
    pub fn file_attributes_generator() -> impl Strategy<Value = Vec<String>> {
        prop::vec![
            prop::sample::select(vec!["readonly", "hidden", "executable", "system"]),
            0..=3,
        ]
    }
    
    /// Generate random timestamps
    pub fn timestamp_generator() -> impl Strategy<Value = String> {
        (2020..=2024u32, 1..=12u32, 1..=28u32, 0..=23u32, 0..=59u32)
            .prop_map(|(year, month, day, hour, minute)| {
                format!("{:04}-{:02}-{:02}T{:02}:{:02}:00Z", year, month, day, hour, minute)
            })
    }
    
    /// Generate random UUIDs
    pub fn uuid_generator() -> impl Strategy<Value = String> {
        prop::sample::select(vec![
            "550e8400-e29b-41d4-a716-446655440000",
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
            "6ba7b811-9dad-11d1-80b4-00c04fd430c8",
            "6ba7b812-9dad-11d1-80b4-00c04fd430c8",
            "6ba7b813-9dad-11d1-80b4-00c04fd430c8",
        ]).prop_map(|s| s.to_string())
    }
    
    /// Generate random JSON content
    pub fn json_content_generator() -> impl Strategy<Value = String> {
        (
            prop::sample::select(vec!["todos", "insights", "patterns", "decisions"]),
            (1..=3usize),
        )
        .prop_map(|(data_type, count)| {
            let mut items = Vec::new();
            for i in 0..count {
                items.push(format!(
                    r#"{{"id": "{}-{:03}", "title": "Generated {} {}", "created_at": "2024-01-{:02}T10:00:00Z"}}"#,
                    data_type, i + 1, data_type, i + 1, (i % 30) + 1
                ));
            }
            format!("{{\"{}\": [{}]}}", data_type, items.join(", "))
        })
    }
    
    /// Generate random XML content
    pub fn xml_content_generator() -> impl Strategy<Value = String> {
        (
            prop::sample::select(vec!["todos", "insights", "patterns", "decisions"]),
            (1..=3usize),
        )
        .prop_map(|(data_type, count)| {
            let mut items = Vec::new();
            for i in 0..count {
                items.push(format!(
                    r#"<item><id>{}-{:03}</id><title>Generated {} {}</title><created_at>2024-01-{:02}T10:00:00Z</created_at></item>"#,
                    data_type, i + 1, data_type, i + 1, (i % 30) + 1
                ));
            }
            format!("<root><{}>{}</{}></root>", data_type, items.join(""), data_type)
        })
    }
    
    /// Generate random TOML content
    pub fn toml_content_generator() -> impl Strategy<Value = String> {
        (
            prop::sample::select(vec!["todos", "insights", "patterns", "decisions"]),
            (1..=3usize),
        )
        .prop_map(|(data_type, count)| {
            let mut content = format!("[[{}]]\n", data_type);
            for i in 0..count {
                content.push_str(&format!("id = \"{}-{:03}\"\n", data_type, i + 1));
                content.push_str(&format!("title = \"Generated {} {}\"\n", data_type, i + 1));
                content.push_str(&format!("created_at = \"2024-01-{:02}T10:00:00Z\"\n", (i % 30) + 1));
                if i < count - 1 {
                    content.push_str(&format!("\n[[{}]]\n", data_type));
                }
            }
            content
        })
    }
    
    /// Generate random binary data
    pub fn binary_data_generator() -> impl Strategy<Value = Vec<u8>> {
        prop::vec![any::<u8>(), 0..=1024]
    }
    
    /// Generate random text data with various encodings
    pub fn text_data_generator() -> impl Strategy<Value = String> {
        prop::string::string_regex("[a-zA-Z0-9\\s\\-_.,!?;:()\"']{1,100}").unwrap()
    }
    
    /// Generate random numeric data
    pub fn numeric_data_generator() -> impl Strategy<Value = f64> {
        -1000.0..1000.0
    }
    
    /// Generate random boolean data
    pub fn boolean_data_generator() -> impl Strategy<Value = bool> {
        prop::bool::ANY
    }
    
    /// Generate random array data
    pub fn array_data_generator() -> impl Strategy<Value = Vec<String>> {
        prop::vec![prop::string::string_regex("[a-zA-Z0-9]{1,10}").unwrap(), 0..=10]
    }
    
    /// Generate random object data
    pub fn object_data_generator() -> impl Strategy<Value = HashMap<String, String>> {
        prop::collection::hash_map(
            prop::string::string_regex("[a-zA-Z0-9_]{1,10}").unwrap(),
            prop::string::string_regex("[a-zA-Z0-9\\s]{1,20}").unwrap(),
            0..=5,
        )
    }
    
    /// Generate random null data
    pub fn null_data_generator() -> impl Strategy<Value = ()> {
        Just(())
    }
    
    /// Generate random complex nested data
    pub fn complex_data_generator() -> impl Strategy<Value = HashMap<String, Value>> {
        (
            prop::string::string_regex("[a-zA-Z0-9_]{1,10}").unwrap(),
            prop::vec![
                (
                    prop::string::string_regex("[a-zA-Z0-9_]{1,10}").unwrap(),
                    prop::oneof![
                        prop::string::string_regex("[a-zA-Z0-9\\s]{1,20}").unwrap().prop_map(Value::String),
                        (0..=1000u32).prop_map(|n| Value::Number(n.into())),
                        prop::bool::ANY.prop_map(Value::Bool),
                        prop::vec![prop::string::string_regex("[a-zA-Z0-9]{1,10}").unwrap(), 0..=5].prop_map(|v| Value::Sequence(v.into_iter().map(Value::String).collect())),
                    ],
                ),
                0..=5,
            ],
        )
        .prop_map(|(key, fields)| {
            let mut obj = HashMap::new();
            obj.insert(key, Value::Mapping(fields.into_iter().collect()));
            obj
        })
    }
} 