//! Performance monitoring examples for rhema-query
//! 
//! This example demonstrates the performance monitoring functionality of rhema-query.

use rhema_query::query::{CqlQuery, Condition, Operator, OrderBy, OrderDirection, ConditionValue, LogicalOperator};
use rhema_core::RhemaResult;

fn main() -> RhemaResult<()> {
    println!("⚡ Rhema Performance Examples");
    println!("============================");

    // Example 1: Basic CQL Query Structure
    println!("\n1. Basic CQL Query Structure");
    println!("-----------------------------");
    
    let query = CqlQuery {
        query: "SELECT todos WHERE status='pending'".to_string(),
        target: "todos".to_string(),
        yaml_path: None,
        conditions: vec![
            Condition {
                field: "status".to_string(),
                operator: Operator::Equals,
                value: ConditionValue::String("pending".to_string()),
                logical_op: LogicalOperator::And,
            },
        ],
        scope_context: None,
        order_by: None,
        limit: Some(10),
        offset: Some(0),
    };
    
    println!("✅ CQL Query created successfully");
    println!("  Target: {}", query.target);
    println!("  Conditions: {}", query.conditions.len());
    println!("  Limit: {:?}", query.limit);

    // Example 2: Complex Query with Multiple Conditions
    println!("\n2. Complex Query with Multiple Conditions");
    println!("------------------------------------------");
    
    let complex_query = CqlQuery {
        query: "SELECT todos WHERE status='pending' AND priority='high' ORDER BY created_at DESC".to_string(),
        target: "todos".to_string(),
        yaml_path: None,
        conditions: vec![
            Condition {
                field: "status".to_string(),
                operator: Operator::Equals,
                value: ConditionValue::String("pending".to_string()),
                logical_op: LogicalOperator::And,
            },
            Condition {
                field: "priority".to_string(),
                operator: Operator::Equals,
                value: ConditionValue::String("high".to_string()),
                logical_op: LogicalOperator::And,
            },
        ],
        scope_context: None,
        order_by: Some(vec![
            OrderBy {
                field: "created_at".to_string(),
                direction: OrderDirection::Desc,
            },
        ]),
        limit: Some(20),
        offset: Some(0),
    };
    
    println!("✅ Complex CQL Query created successfully");
    println!("  Conditions: {}", complex_query.conditions.len());
    println!("  Order by: {:?}", complex_query.order_by.as_ref().map(|o| o.len()));

    // Example 3: Query with YAML Path
    println!("\n3. Query with YAML Path");
    println!("----------------------");
    
    let yaml_path_query = CqlQuery {
        query: "SELECT todos.title, todos.description FROM scope('user-service').todos".to_string(),
        target: "todos".to_string(),
        yaml_path: Some("todos.title, todos.description".to_string()),
        conditions: vec![],
        scope_context: Some("user-service".to_string()),
        order_by: None,
        limit: None,
        offset: None,
    };
    
    println!("✅ YAML Path Query created successfully");
    println!("  YAML Path: {:?}", yaml_path_query.yaml_path);
    println!("  Scope Context: {:?}", yaml_path_query.scope_context);

    // Example 4: Query with Different Operators
    println!("\n4. Query with Different Operators");
    println!("---------------------------------");
    
    let operators_query = CqlQuery {
        query: "SELECT todos WHERE priority > 5 AND status IN ('pending', 'in_progress')".to_string(),
        target: "todos".to_string(),
        yaml_path: None,
        conditions: vec![
            Condition {
                field: "priority".to_string(),
                operator: Operator::GreaterThan,
                value: ConditionValue::Number(5.0),
                logical_op: LogicalOperator::And,
            },
            Condition {
                field: "status".to_string(),
                operator: Operator::In,
                value: ConditionValue::Array(vec![
                    ConditionValue::String("pending".to_string()),
                    ConditionValue::String("in_progress".to_string()),
                ]),
                logical_op: LogicalOperator::And,
            },
        ],
        scope_context: None,
        order_by: None,
        limit: Some(50),
        offset: None,
    };
    
    println!("✅ Operators Query created successfully");
    println!("  Conditions: {}", operators_query.conditions.len());
    println!("  First condition operator: {:?}", operators_query.conditions[0].operator);

    // Example 5: Query with Logical Operators
    println!("\n5. Query with Logical Operators");
    println!("-------------------------------");
    
    let logical_query = CqlQuery {
        query: "SELECT todos WHERE (status='pending' OR status='in_progress') AND priority='high'".to_string(),
        target: "todos".to_string(),
        yaml_path: None,
        conditions: vec![
            Condition {
                field: "status".to_string(),
                operator: Operator::Equals,
                value: ConditionValue::String("pending".to_string()),
                logical_op: LogicalOperator::Or,
            },
            Condition {
                field: "status".to_string(),
                operator: Operator::Equals,
                value: ConditionValue::String("in_progress".to_string()),
                logical_op: LogicalOperator::Or,
            },
            Condition {
                field: "priority".to_string(),
                operator: Operator::Equals,
                value: ConditionValue::String("high".to_string()),
                logical_op: LogicalOperator::And,
            },
        ],
        scope_context: None,
        order_by: None,
        limit: Some(10),
        offset: None,
    };
    
    println!("✅ Logical Operators Query created successfully");
    println!("  Conditions: {}", logical_query.conditions.len());
    println!("  Logical operators: {:?}", logical_query.conditions.iter().map(|c| &c.logical_op).collect::<Vec<_>>());

    println!("\n🎉 All performance examples completed!");
    Ok(())
}
