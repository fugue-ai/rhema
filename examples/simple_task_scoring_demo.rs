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

use rhema_ai::agent::task_scoring::*;
use chrono::Utc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Simple Task Scoring Demo");
    println!("==========================");

    // Create a task scoring system
    let mut system = TaskScoringSystem::new();
    println!("✅ Task scoring system created");

    // Create some sample tasks
    let tasks = vec![
        Task {
            id: "task-001".to_string(),
            title: "Implement User Authentication".to_string(),
            description: "Add secure user authentication with JWT tokens".to_string(),
            task_type: TaskType::Feature,
            priority: TaskPriority::High,
            status: TaskStatus::Pending,
            complexity: TaskComplexity::Moderate,
            scoring_factors: TaskScoringFactors {
                business_value: 0.9,
                technical_debt_impact: 0.2,
                user_impact: 0.8,
                dependencies_count: 1,
                estimated_effort_hours: 16.0,
                risk_level: 0.3,
                urgency: 0.7,
                team_capacity_impact: 0.5,
                learning_value: 0.6,
                strategic_alignment: 0.8,
            },
            scope: "backend".to_string(),
            assigned_to: None,
            dependencies: vec![],
            blocking: vec![],
            related_tasks: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
            due_date: None,
            estimated_completion: None,
            completed_at: None,
            metadata: HashMap::new(),
        },
        Task {
            id: "task-002".to_string(),
            title: "Fix Database Performance Issue".to_string(),
            description: "Optimize slow database queries and add indexes".to_string(),
            task_type: TaskType::Performance,
            priority: TaskPriority::Critical,
            status: TaskStatus::Pending,
            complexity: TaskComplexity::Complex,
            scoring_factors: TaskScoringFactors {
                business_value: 0.7,
                technical_debt_impact: 0.8,
                user_impact: 0.9,
                dependencies_count: 0,
                estimated_effort_hours: 24.0,
                risk_level: 0.4,
                urgency: 0.9,
                team_capacity_impact: 0.6,
                learning_value: 0.7,
                strategic_alignment: 0.6,
            },
            scope: "backend".to_string(),
            assigned_to: None,
            dependencies: vec![],
            blocking: vec![],
            related_tasks: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
            due_date: None,
            estimated_completion: None,
            completed_at: None,
            metadata: HashMap::new(),
        },
        Task {
            id: "task-003".to_string(),
            title: "Update API Documentation".to_string(),
            description: "Update OpenAPI documentation for new endpoints".to_string(),
            task_type: TaskType::Documentation,
            priority: TaskPriority::Normal,
            status: TaskStatus::Pending,
            complexity: TaskComplexity::Simple,
            scoring_factors: TaskScoringFactors {
                business_value: 0.4,
                technical_debt_impact: 0.1,
                user_impact: 0.3,
                dependencies_count: 2,
                estimated_effort_hours: 4.0,
                risk_level: 0.1,
                urgency: 0.3,
                team_capacity_impact: 0.2,
                learning_value: 0.2,
                strategic_alignment: 0.4,
            },
            scope: "backend".to_string(),
            assigned_to: None,
            dependencies: vec!["task-001".to_string()],
            blocking: vec![],
            related_tasks: vec![],
            created_at: Utc::now(),
            modified_at: Utc::now(),
            due_date: None,
            estimated_completion: None,
            completed_at: None,
            metadata: HashMap::new(),
        },
    ];

    // Add tasks to the system
    for task in tasks {
        system.add_task(task)?;
    }
    println!("✅ Added {} tasks to the system", system.get_scope_tasks("backend").len());

    // Calculate individual task scores
    println!("\n📊 Individual Task Scores:");
    println!("==========================");
    
    for task in system.get_scope_tasks("backend") {
        let score = system.calculate_task_score(&task.id)?;
        println!("Task: {}", task.title);
        println!("  Overall Score: {:.3}", score.overall_score);
        println!("  Business Value: {:.3}", score.business_value_score);
        println!("  User Impact: {:.3}", score.user_impact_score);
        println!("  Technical Debt: {:.3}", score.technical_debt_score);
        println!("  Risk Adjusted: {:.3}", score.risk_adjusted_score);
        println!("  Explanation: {}", score.explanation);
        println!();
    }

    // Prioritize tasks using different strategies
    println!("🎯 Task Prioritization Results:");
    println!("==============================");

    let strategies = vec![
        PrioritizationStrategy::WeightedScoring,
        PrioritizationStrategy::BusinessValueFirst,
        PrioritizationStrategy::TechnicalDebtFirst,
        PrioritizationStrategy::UserImpactFirst,
        PrioritizationStrategy::RiskAdjustedReturn,
    ];

    for strategy in strategies {
        let prioritization = system.prioritize_tasks("backend", strategy.clone())?;
        
        println!("\nStrategy: {:?}", strategy);
        println!("Top 3 Tasks:");
        for (i, task_score) in prioritization.prioritized_tasks.iter().take(3).enumerate() {
            let task = system.get_task(&task_score.task_id).unwrap();
            println!("  {}. {} (Score: {:.3})", i + 1, task.title, task_score.overall_score);
        }
        
        println!("Recommendations:");
        for recommendation in &prioritization.recommendations {
            println!("  • {}", recommendation);
        }
        
        println!("Stats: {} tasks processed, avg score: {:.3}", 
            prioritization.stats.total_tasks, 
            prioritization.stats.average_score);
    }

    println!("\n🎉 Task Scoring Demo Completed Successfully!");
    Ok(())
} 