use chrono::{Utc, Duration};
use rhema_ai::agent::conflict_analysis::{
    ConflictAnalysisSystem, ConflictAnalysisConfig, ReportType, AnalysisPeriod,
    PriorityLevel, EffortLevel, TrendDirection
};
use rhema_ai::agent::conflict_prevention::{
    Conflict, ConflictType, ConflictSeverity, ConflictStatus, ConflictDetails,
    ConflictResolution, ResolutionStrategy, ResolutionAction, ResolutionMetrics
};
use rhema_ai::agent::ml_conflict_prediction::{
    ConflictPredictionResult, MLConflictPredictionStats, LearningMetrics
};
use rhema_ai::agent::real_time_coordination::RealTimeCoordinationSystem;
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::test]
async fn test_conflict_analysis_system_creation() {
    let coordination_system = Arc::new(RealTimeCoordinationSystem::new());
    let config = ConflictAnalysisConfig::default();
    
    let analysis_system = ConflictAnalysisSystem::new(coordination_system, config).await.unwrap();
    
    assert!(analysis_system.get_reports().await.is_empty());
}

#[tokio::test]
async fn test_conflict_analysis_basic_functionality() {
    let coordination_system = Arc::new(RealTimeCoordinationSystem::new());
    let config = ConflictAnalysisConfig::default();
    let analysis_system = ConflictAnalysisSystem::new(coordination_system, config).await.unwrap();
    
    // Add some test conflicts
    let conflict1 = Conflict {
        id: "conflict_1".to_string(),
        conflict_type: ConflictType::FileModification,
        severity: ConflictSeverity::Error,
        status: ConflictStatus::Detected,
        description: "Test conflict 1".to_string(),
        involved_agents: vec!["agent_1".to_string(), "agent_2".to_string()],
        affected_scope: "test_scope".to_string(),
        detected_at: Utc::now() - Duration::hours(1),
        resolved_at: None,
        resolution_strategy: None,
        resolution_notes: None,
        details: ConflictDetails {
            file_modification: None,
            dependency: None,
            resource: None,
            custom: None,
        },
        metadata: HashMap::new(),
    };
    
    let conflict2 = Conflict {
        id: "conflict_2".to_string(),
        conflict_type: ConflictType::Dependency,
        severity: ConflictSeverity::Warning,
        status: ConflictStatus::Resolved,
        description: "Test conflict 2".to_string(),
        involved_agents: vec!["agent_1".to_string()],
        affected_scope: "test_scope_2".to_string(),
        detected_at: Utc::now() - Duration::hours(2),
        resolved_at: Some(Utc::now() - Duration::hours(1)),
        resolution_strategy: Some(ResolutionStrategy::Automatic),
        resolution_notes: Some("Auto-resolved".to_string()),
        details: ConflictDetails {
            file_modification: None,
            dependency: None,
            resource: None,
            custom: None,
        },
        metadata: HashMap::new(),
    };
    
    analysis_system.add_conflict(conflict1).await.unwrap();
    analysis_system.add_conflict(conflict2).await.unwrap();
    
    // Add test resolutions
    let resolution1 = ConflictResolution {
        conflict_id: "conflict_1".to_string(),
        strategy: ResolutionStrategy::Automatic,
        timestamp: Utc::now() - Duration::hours(1),
        description: "Auto-resolved conflict".to_string(),
        actions: vec![
            ResolutionAction {
                action_type: "auto_resolve".to_string(),
                description: "Automatic resolution".to_string(),
                timestamp: Utc::now() - Duration::hours(1),
                performed_by: "system".to_string(),
                result: "success".to_string(),
            }
        ],
        successful: true,
        metrics: ResolutionMetrics {
            time_to_resolution_seconds: 300,
            agents_involved: 1,
            complexity_score: 0.3,
            satisfaction_score: 0.8,
        },
    };
    
    analysis_system.add_resolution(resolution1).await.unwrap();
    
    // Add test predictions
    let prediction1 = ConflictPredictionResult {
        id: "pred_1".to_string(),
        conflict_probability: 0.8,
        confidence: 0.9,
        predicted_conflict_type: Some(ConflictType::FileModification),
        predicted_severity: Some(ConflictSeverity::Error),
        predicted_agents: vec!["agent_1".to_string()],
        features_used: HashMap::new(),
        prediction_reason: "High probability of file conflict".to_string(),
        mitigation_suggestions: vec!["Coordinate file access".to_string()],
        prevention_actions: vec![],
        timestamp: Utc::now() - Duration::hours(1),
    };
    
    analysis_system.add_prediction(prediction1).await.unwrap();
    
    // Update ML stats
    let ml_stats = MLConflictPredictionStats {
        total_models: 2,
        active_models: 2,
        total_predictions: 1,
        total_conflicts: 1,
        learning_metrics: LearningMetrics {
            total_samples: 100,
            successful_predictions: 85,
            failed_predictions: 15,
            accuracy_improvement: 0.05,
            last_update: Utc::now(),
        },
        last_retraining: Utc::now(),
    };
    
    analysis_system.update_ml_stats(ml_stats).await.unwrap();
    
    // Generate a comprehensive report
    let report = analysis_system.generate_analysis_report(
        ReportType::Detailed,
        Utc::now() - Duration::hours(3),
        Utc::now(),
    ).await.unwrap();
    
    // Verify the report was generated successfully
    assert_eq!(report.report_type, ReportType::Detailed);
    assert!(report.data.conflict_stats.total_conflicts > 0);
    assert!(report.data.resolution_stats.total_resolutions > 0);
    assert!(report.data.prediction_stats.total_predictions > 0);
    assert!(!report.data.recommendations.is_empty());
    
    // Test report retrieval
    let retrieved_report = analysis_system.get_report(&report.id).await;
    assert!(retrieved_report.is_some());
    let retrieved_report = retrieved_report.unwrap();
    assert_eq!(retrieved_report.id, report.id);
    assert_eq!(retrieved_report.report_type, report.report_type);
    
    // Test report export
    let json_export = analysis_system.export_report_json(&report.id).await.unwrap();
    assert!(!json_export.is_empty());
    assert!(json_export.contains(&report.id));
    assert!(json_export.contains("conflict_stats"));
    assert!(json_export.contains("resolution_stats"));
    assert!(json_export.contains("prediction_stats"));
}

#[tokio::test]
async fn test_analysis_period_duration() {
    let period = AnalysisPeriod {
        start_time: Utc::now() - Duration::hours(24),
        end_time: Utc::now(),
        duration_hours: 24,
    };
    
    assert_eq!(period.duration_days(), 1.0);
    
    let period = AnalysisPeriod {
        start_time: Utc::now() - Duration::hours(48),
        end_time: Utc::now(),
        duration_hours: 48,
    };
    
    assert_eq!(period.duration_days(), 2.0);
}

#[tokio::test]
async fn test_report_management() {
    let coordination_system = Arc::new(RealTimeCoordinationSystem::new());
    let config = ConflictAnalysisConfig::default();
    let analysis_system = ConflictAnalysisSystem::new(coordination_system, config).await.unwrap();
    
    // Generate multiple reports
    let report1 = analysis_system.generate_analysis_report(
        ReportType::Summary,
        Utc::now() - Duration::hours(1),
        Utc::now(),
    ).await.unwrap();
    
    let report2 = analysis_system.generate_analysis_report(
        ReportType::Detailed,
        Utc::now() - Duration::hours(1),
        Utc::now(),
    ).await.unwrap();
    
    // Get all reports
    let reports = analysis_system.get_reports().await;
    assert_eq!(reports.len(), 2);
    
    // Get specific report
    let retrieved_report = analysis_system.get_report(&report1.id).await;
    assert!(retrieved_report.is_some());
    assert_eq!(retrieved_report.unwrap().id, report1.id);
    
    // Test non-existent report
    let non_existent = analysis_system.get_report("non-existent-id").await;
    assert!(non_existent.is_none());
    
    // Export report to JSON
    let json_export = analysis_system.export_report_json(&report1.id).await.unwrap();
    assert!(json_export.contains(&report1.id));
    assert!(json_export.contains("Summary"));
}

#[tokio::test]
async fn test_high_severity_conflict_analysis() {
    let coordination_system = Arc::new(RealTimeCoordinationSystem::new());
    let config = ConflictAnalysisConfig::default();
    let analysis_system = ConflictAnalysisSystem::new(coordination_system, config).await.unwrap();
    
    // Add high severity conflict to trigger recommendations
    let conflict = Conflict {
        id: "high_conflict".to_string(),
        conflict_type: ConflictType::FileModification,
        severity: ConflictSeverity::Critical,
        status: ConflictStatus::Detected,
        description: "High severity conflict".to_string(),
        involved_agents: vec!["agent_1".to_string(), "agent_2".to_string()],
        affected_scope: "critical_scope".to_string(),
        detected_at: Utc::now() - Duration::hours(1),
        resolved_at: None,
        resolution_strategy: None,
        resolution_notes: None,
        details: ConflictDetails {
            file_modification: None,
            dependency: None,
            resource: None,
            custom: None,
        },
        metadata: HashMap::new(),
    };
    
    analysis_system.add_conflict(conflict).await.unwrap();
    
    let period = AnalysisPeriod {
        start_time: Utc::now() - Duration::hours(2),
        end_time: Utc::now(),
        duration_hours: 2,
    };
    
    let report = analysis_system.generate_analysis_report(
        ReportType::Detailed,
        period.start_time,
        period.end_time,
    ).await.unwrap();
    
    // Check that recommendations were generated
    assert!(!report.data.recommendations.is_empty());
    
    let recommendation = &report.data.recommendations[0];
    assert!(!recommendation.id.is_empty());
    assert!(!recommendation.title.is_empty());
    assert!(!recommendation.description.is_empty());
    assert!(recommendation.priority >= PriorityLevel::Low);
    assert!(recommendation.implementation_effort >= EffortLevel::Minimal);
    
    // Check impact assessment
    assert!(recommendation.expected_impact.conflict_reduction_impact >= 0.0);
    assert!(recommendation.expected_impact.conflict_reduction_impact <= 1.0);
    assert!(recommendation.expected_impact.resolution_time_impact >= 0.0);
    assert!(recommendation.expected_impact.resolution_time_impact <= 1.0);
    assert!(recommendation.expected_impact.prediction_accuracy_impact >= 0.0);
    assert!(recommendation.expected_impact.prediction_accuracy_impact <= 1.0);
    assert!(recommendation.expected_impact.overall_impact >= 0.0);
    assert!(recommendation.expected_impact.overall_impact <= 1.0);
} 