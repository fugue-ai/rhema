use tempfile::TempDir;
use std::process::Command;
use rhema::Rhema;

#[test]
fn test_basic_cql_functionality() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    
    // Initialize git repository
    Command::new("git")
        .args(["init"])
        .current_dir(temp_path)
        .output()
        .unwrap();
    
    // Create .rhema directory
    let rhema_dir = temp_path.join(".rhema");
    std::fs::create_dir_all(&rhema_dir).unwrap();
    
    // Create scope definition file
    let scope_content = r#"
name: "test-scope"
scope_type: "service"
description: "Test scope for CQL improvements"
version: "1.0.0"
schema_version: "1.0.0"
"#;
    let scope_file = rhema_dir.join("rhema.yaml");
    std::fs::write(&scope_file, scope_content).unwrap();
    
    // Create a simple YAML file
    let yaml_content = r#"
items:
  - name: item1
    value: 10
    active: true
  - name: item2
    value: 20
    active: false
  - name: item3
    value: 15
    active: true
"#;
    
    let simple_file = rhema_dir.join("simple.yaml");
    std::fs::write(&simple_file, yaml_content).unwrap();
    
    let rhema = Rhema::new_from_path(temp_path.to_path_buf()).unwrap();
    
    // Test basic query
    let result = rhema.query("simple").unwrap();
    let result_str = serde_yaml::to_string(&result).unwrap();
    println!("Basic query result: {}", result_str);
    
    // Test WHERE query
    let where_result = rhema.query("simple.items WHERE active=true").unwrap();
    let where_result_str = serde_yaml::to_string(&where_result).unwrap();
    println!("WHERE query result: {}", where_result_str);
    

    

    
    assert!(result_str.contains("item1"));
    assert!(where_result_str.contains("item1"));
    assert!(where_result_str.contains("item3"));
    assert!(!where_result_str.contains("item2")); // item2 has active=false
} 

#[test]
fn test_query_parsing() {
    use rhema::query::parse_cql_query;
    
    // Test parsing a simple query
    let query = "simple.items WHERE active=true";
    let parsed = parse_cql_query(query).unwrap();
    
    println!("Parsed query: {:?}", parsed);
    assert_eq!(parsed.target, "simple");
    assert_eq!(parsed.yaml_path, Some("items".to_string()));
    assert_eq!(parsed.conditions.len(), 1);
    assert_eq!(parsed.conditions[0].field, "active");
    
    // Test parsing a query with comparison
    let query = "simple.items WHERE value>15";
    let parsed = parse_cql_query(query).unwrap();
    
    println!("Parsed comparison query: {:?}", parsed);
    assert_eq!(parsed.target, "simple");
    assert_eq!(parsed.yaml_path, Some("items".to_string()));
    assert_eq!(parsed.conditions.len(), 1);
    assert_eq!(parsed.conditions[0].field, "value");
    
    println!("Query parsing test completed successfully!");
} 