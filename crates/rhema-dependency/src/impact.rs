use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::error::{Error, ImpactAnalysisResult, Result, RiskLevel};
use crate::graph::DependencyGraph;
use crate::types::{DependencyConfig, DependencyType, HealthMetrics, HealthStatus, ImpactScore};

/// Business impact metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessImpactMetrics {
    /// Revenue impact weight (0.0 to 1.0)
    pub revenue_weight: f64,
    /// User experience impact weight (0.0 to 1.0)
    pub user_experience_weight: f64,
    /// Operational cost impact weight (0.0 to 1.0)
    pub operational_cost_weight: f64,
    /// Security impact weight (0.0 to 1.0)
    pub security_weight: f64,
    /// Compliance impact weight (0.0 to 1.0)
    pub compliance_weight: f64,
    /// Brand reputation impact weight (0.0 to 1.0)
    pub brand_reputation_weight: f64,
}

impl Default for BusinessImpactMetrics {
    fn default() -> Self {
        Self {
            revenue_weight: 0.3,
            user_experience_weight: 0.25,
            operational_cost_weight: 0.2,
            security_weight: 0.15,
            compliance_weight: 0.05,
            brand_reputation_weight: 0.05,
        }
    }
}

/// Risk factors configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactors {
    /// Availability risk weight (0.0 to 1.0)
    pub availability_weight: f64,
    /// Performance risk weight (0.0 to 1.0)
    pub performance_weight: f64,
    /// Security risk weight (0.0 to 1.0)
    pub security_weight: f64,
    /// Scalability risk weight (0.0 to 1.0)
    pub scalability_weight: f64,
    /// Maintainability risk weight (0.0 to 1.0)
    pub maintainability_weight: f64,
    /// Compliance risk weight (0.0 to 1.0)
    pub compliance_weight: f64,
}

impl Default for RiskFactors {
    fn default() -> Self {
        Self {
            availability_weight: 0.3,
            performance_weight: 0.25,
            security_weight: 0.2,
            scalability_weight: 0.15,
            maintainability_weight: 0.05,
            compliance_weight: 0.05,
        }
    }
}

/// Impact analysis engine
pub struct ImpactAnalysis {
    /// Business impact metrics configuration
    business_metrics: BusinessImpactMetrics,
    /// Risk factors configuration
    risk_factors: RiskFactors,
    /// Critical business functions mapping
    critical_functions: HashMap<String, Vec<String>>,
    /// Cost per hour of downtime for different dependency types
    downtime_costs: HashMap<DependencyType, f64>,
    /// Historical impact data
    historical_impacts: HashMap<String, Vec<ImpactAnalysisResult>>,
}

impl ImpactAnalysis {
    /// Create a new impact analysis engine
    pub fn new() -> Self {
        Self {
            business_metrics: BusinessImpactMetrics::default(),
            risk_factors: RiskFactors::default(),
            critical_functions: HashMap::new(),
            downtime_costs: Self::default_downtime_costs(),
            historical_impacts: HashMap::new(),
        }
    }

    /// Create a new impact analysis engine with custom configuration
    pub fn with_config(business_metrics: BusinessImpactMetrics, risk_factors: RiskFactors) -> Self {
        Self {
            business_metrics,
            risk_factors,
            critical_functions: HashMap::new(),
            downtime_costs: Self::default_downtime_costs(),
            historical_impacts: HashMap::new(),
        }
    }

    /// Set business impact metrics
    pub fn with_business_metrics(mut self, metrics: BusinessImpactMetrics) -> Self {
        self.business_metrics = metrics;
        self
    }

    /// Set risk factors
    pub fn with_risk_factors(mut self, factors: RiskFactors) -> Self {
        self.risk_factors = factors;
        self
    }

    /// Add critical business functions
    pub fn add_critical_functions(mut self, dependency_id: String, functions: Vec<String>) -> Self {
        self.critical_functions.insert(dependency_id, functions);
        self
    }

    /// Set downtime costs for dependency types
    pub fn set_downtime_costs(mut self, costs: HashMap<DependencyType, f64>) -> Self {
        self.downtime_costs = costs;
        self
    }

    /// Analyze the impact of a dependency failure
    pub fn analyze_dependency_impact(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<ImpactAnalysisResult> {
        let config = graph.get_dependency_config(dependency_id)?;

        // Calculate business impact score
        let business_impact_score = self.calculate_business_impact_score(dependency_id, graph)?;

        // Calculate risk level
        let risk_level = self.calculate_risk_level(dependency_id, graph)?;

        // Find affected services
        let affected_services = self.find_affected_services(dependency_id, graph)?;

        // Estimate downtime
        let estimated_downtime = self.estimate_downtime(dependency_id, graph)?;

        // Calculate cost impact
        let cost_impact = self.calculate_cost_impact(dependency_id, estimated_downtime, graph)?;

        let result = ImpactAnalysisResult {
            business_impact_score,
            risk_level,
            affected_services,
            estimated_downtime,
            cost_impact,
            timestamp: Utc::now(),
        };

        // Store historical data
        self.store_historical_impact(dependency_id, result.clone());

        Ok(result)
    }

    /// Analyze the impact of a proposed change
    pub fn analyze_change_impact(
        &self,
        change_description: &str,
        affected_dependencies: &[String],
        graph: &DependencyGraph,
    ) -> Result<ImpactAnalysisResult> {
        let mut total_business_impact = 0.0;
        let mut total_risk_score = 0.0;
        let mut all_affected_services = HashSet::new();
        let mut total_downtime = std::time::Duration::from_secs(0);
        let mut total_cost_impact = 0.0;

        for dependency_id in affected_dependencies {
            let impact = self.analyze_dependency_impact(dependency_id, graph)?;
            total_business_impact += impact.business_impact_score;
            total_risk_score += match impact.risk_level {
                crate::RiskLevel::Low => 0.25,
                crate::RiskLevel::Medium => 0.5,
                crate::RiskLevel::High => 0.75,
                crate::RiskLevel::Critical => 1.0,
            };
            all_affected_services.extend(impact.affected_services);
            total_downtime += impact.estimated_downtime;
            total_cost_impact += impact.cost_impact;
        }

        let avg_business_impact = total_business_impact / affected_dependencies.len() as f64;
        let avg_risk_level = RiskLevel::from(total_risk_score / affected_dependencies.len() as f64);

        Ok(ImpactAnalysisResult {
            business_impact_score: avg_business_impact,
            risk_level: avg_risk_level,
            affected_services: all_affected_services.into_iter().collect(),
            estimated_downtime: total_downtime,
            cost_impact: total_cost_impact,
            timestamp: Utc::now(),
        })
    }

    /// Calculate business impact score
    fn calculate_business_impact_score(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<f64> {
        let config = graph.get_dependency_config(dependency_id)?;

        // Base impact based on dependency type
        let base_impact = self.get_base_impact_for_type(&config.dependency_type);

        // Critical functions impact
        let critical_impact = self.calculate_critical_functions_impact(dependency_id);

        // Dependency depth impact (how many services depend on this)
        let depth_impact = self.calculate_dependency_depth_impact(dependency_id, graph)?;

        // Health status impact
        let health_impact = self.calculate_health_impact(dependency_id, graph)?;

        // Weighted combination
        let total_impact =
            base_impact * 0.3 + critical_impact * 0.3 + depth_impact * 0.2 + health_impact * 0.2;

        Ok(total_impact.min(1.0))
    }

    /// Calculate risk level
    fn calculate_risk_level(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<RiskLevel> {
        let config = graph.get_dependency_config(dependency_id)?;

        // Availability risk
        let availability_risk = self.calculate_availability_risk(dependency_id, graph)?;

        // Performance risk
        let performance_risk = self.calculate_performance_risk(dependency_id, graph)?;

        // Security risk
        let security_risk = self.calculate_security_risk(dependency_id, graph)?;

        // Scalability risk
        let scalability_risk = self.calculate_scalability_risk(dependency_id, graph)?;

        // Maintainability risk
        let maintainability_risk = self.calculate_maintainability_risk(dependency_id, graph)?;

        // Compliance risk
        let compliance_risk = self.calculate_compliance_risk(dependency_id, graph)?;

        let total_risk = availability_risk * self.risk_factors.availability_weight
            + performance_risk * self.risk_factors.performance_weight
            + security_risk * self.risk_factors.security_weight
            + scalability_risk * self.risk_factors.scalability_weight
            + maintainability_risk * self.risk_factors.maintainability_weight
            + compliance_risk * self.risk_factors.compliance_weight;

        Ok(RiskLevel::from(total_risk))
    }

    /// Find affected services
    fn find_affected_services(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<Vec<String>> {
        let mut affected = HashSet::new();
        let mut to_visit = vec![dependency_id.to_string()];
        let mut visited = HashSet::new();

        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            // Add current dependency
            affected.insert(current.clone());

            // Add all dependencies that depend on this one
            let dependents = graph.get_dependents(&current)?;
            for dependent in dependents {
                if !visited.contains(&dependent) {
                    to_visit.push(dependent);
                }
            }
        }

        Ok(affected.into_iter().collect())
    }

    /// Estimate downtime
    fn estimate_downtime(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<std::time::Duration> {
        let config = graph.get_dependency_config(dependency_id)?;

        // Base downtime based on dependency type
        let base_downtime = match config.dependency_type {
            DependencyType::DataFlow => std::time::Duration::from_secs(300), // 5 minutes
            DependencyType::ApiCall => std::time::Duration::from_secs(60),   // 1 minute
            DependencyType::Infrastructure => std::time::Duration::from_secs(1800), // 30 minutes
            DependencyType::BusinessLogic => std::time::Duration::from_secs(600), // 10 minutes
            DependencyType::Security => std::time::Duration::from_secs(120), // 2 minutes
            DependencyType::Monitoring => std::time::Duration::from_secs(300), // 5 minutes
            DependencyType::Configuration => std::time::Duration::from_secs(60), // 1 minute
            DependencyType::Deployment => std::time::Duration::from_secs(900), // 15 minutes
        };

        // Adjust based on health status
        let health_multiplier = match graph.get_dependency_config(dependency_id) {
            Ok(_) => {
                // This would need access to health status from the graph
                // For now, use a default multiplier
                1.0
            }
            Err(_) => 1.0,
        };

        Ok(base_downtime.mul_f64(health_multiplier))
    }

    /// Calculate cost impact
    fn calculate_cost_impact(
        &self,
        dependency_id: &str,
        downtime: std::time::Duration,
        graph: &DependencyGraph,
    ) -> Result<f64> {
        let config = graph.get_dependency_config(dependency_id)?;
        let cost_per_hour = self
            .downtime_costs
            .get(&config.dependency_type)
            .unwrap_or(&1000.0);

        let hours = downtime.as_secs_f64() / 3600.0;
        Ok(cost_per_hour * hours)
    }

    /// Get base impact for dependency type
    fn get_base_impact_for_type(&self, dependency_type: &DependencyType) -> f64 {
        match dependency_type {
            DependencyType::DataFlow => 0.8,
            DependencyType::ApiCall => 0.7,
            DependencyType::Infrastructure => 0.9,
            DependencyType::BusinessLogic => 0.6,
            DependencyType::Security => 0.9,
            DependencyType::Monitoring => 0.5,
            DependencyType::Configuration => 0.4,
            DependencyType::Deployment => 0.6,
        }
    }

    /// Calculate critical functions impact
    fn calculate_critical_functions_impact(&self, dependency_id: &str) -> f64 {
        self.critical_functions
            .get(dependency_id)
            .map(|functions| {
                if functions.is_empty() {
                    0.3
                } else {
                    0.3 + (functions.len() as f64 * 0.1).min(0.7)
                }
            })
            .unwrap_or(0.3)
    }

    /// Calculate dependency depth impact
    fn calculate_dependency_depth_impact(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<f64> {
        let dependents = graph.get_dependents(dependency_id)?;
        let depth = dependents.len();

        Ok((depth as f64 * 0.05).min(0.5))
    }

    /// Calculate health impact
    fn calculate_health_impact(&self, dependency_id: &str, graph: &DependencyGraph) -> Result<f64> {
        // This would need access to health status from the graph
        // For now, return a default value
        Ok(0.5)
    }

    /// Calculate availability risk
    fn calculate_availability_risk(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<f64> {
        // This would need access to health metrics from the graph
        // For now, return a default value
        Ok(0.3)
    }

    /// Calculate performance risk
    fn calculate_performance_risk(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<f64> {
        // This would need access to performance metrics from the graph
        // For now, return a default value
        Ok(0.3)
    }

    /// Calculate security risk
    fn calculate_security_risk(&self, dependency_id: &str, graph: &DependencyGraph) -> Result<f64> {
        let config = graph.get_dependency_config(dependency_id)?;

        match config.dependency_type {
            DependencyType::Security => Ok(0.8),
            DependencyType::DataFlow => Ok(0.6),
            DependencyType::ApiCall => Ok(0.5),
            _ => Ok(0.3),
        }
    }

    /// Calculate scalability risk
    fn calculate_scalability_risk(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<f64> {
        // This would need access to scalability metrics from the graph
        // For now, return a default value
        Ok(0.3)
    }

    /// Calculate maintainability risk
    fn calculate_maintainability_risk(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<f64> {
        // This would need access to maintainability metrics from the graph
        // For now, return a default value
        Ok(0.3)
    }

    /// Calculate compliance risk
    fn calculate_compliance_risk(
        &self,
        dependency_id: &str,
        graph: &DependencyGraph,
    ) -> Result<f64> {
        let config = graph.get_dependency_config(dependency_id)?;

        match config.dependency_type {
            DependencyType::Security => Ok(0.7),
            DependencyType::DataFlow => Ok(0.5),
            _ => Ok(0.3),
        }
    }

    /// Store historical impact data
    fn store_historical_impact(&self, dependency_id: &str, impact: ImpactAnalysisResult) {
        // This would store the impact data in a database or file
        // For now, just log it
        tracing::info!(
            "Stored historical impact for {}: business_impact={}, risk_level={:?}, cost_impact={}",
            dependency_id,
            impact.business_impact_score,
            impact.risk_level,
            impact.cost_impact
        );
    }

    /// Get default downtime costs
    fn default_downtime_costs() -> HashMap<DependencyType, f64> {
        let mut costs = HashMap::new();
        costs.insert(DependencyType::DataFlow, 5000.0);
        costs.insert(DependencyType::ApiCall, 2000.0);
        costs.insert(DependencyType::Infrastructure, 10000.0);
        costs.insert(DependencyType::BusinessLogic, 3000.0);
        costs.insert(DependencyType::Security, 15000.0);
        costs.insert(DependencyType::Monitoring, 1000.0);
        costs.insert(DependencyType::Configuration, 500.0);
        costs.insert(DependencyType::Deployment, 2000.0);
        costs
    }

    /// Get historical impact data
    pub fn get_historical_impacts(&self, dependency_id: &str) -> Vec<ImpactAnalysisResult> {
        self.historical_impacts
            .get(dependency_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get trend analysis
    pub fn get_trend_analysis(&self, dependency_id: &str) -> Result<TrendAnalysis> {
        let impacts = self.get_historical_impacts(dependency_id);

        if impacts.len() < 2 {
            return Err(Error::ImpactAnalysisFailed(
                "Insufficient historical data".to_string(),
            ));
        }

        let recent_impacts: Vec<_> = impacts.iter().rev().take(10).collect();
        let avg_recent_impact = recent_impacts
            .iter()
            .map(|i| i.business_impact_score)
            .sum::<f64>()
            / recent_impacts.len() as f64;

        let older_impacts: Vec<_> = impacts.iter().rev().skip(10).take(10).collect();
        let avg_older_impact = if older_impacts.is_empty() {
            avg_recent_impact
        } else {
            older_impacts
                .iter()
                .map(|i| i.business_impact_score)
                .sum::<f64>()
                / older_impacts.len() as f64
        };

        let trend = if avg_recent_impact > avg_older_impact * 1.1 {
            TrendDirection::Increasing
        } else if avg_recent_impact < avg_older_impact * 0.9 {
            TrendDirection::Decreasing
        } else {
            TrendDirection::Stable
        };

        Ok(TrendAnalysis {
            dependency_id: dependency_id.to_string(),
            trend,
            recent_average: avg_recent_impact,
            older_average: avg_older_impact,
            change_percentage: ((avg_recent_impact - avg_older_impact) / avg_older_impact) * 100.0,
        })
    }
}

/// Trend analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    /// Dependency ID
    pub dependency_id: String,
    /// Trend direction
    pub trend: TrendDirection,
    /// Recent average impact score
    pub recent_average: f64,
    /// Older average impact score
    pub older_average: f64,
    /// Change percentage
    pub change_percentage: f64,
}

/// Trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
}

impl Default for ImpactAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DependencyConfig;

    fn create_test_config(
        id: &str,
        name: &str,
        dependency_type: DependencyType,
    ) -> DependencyConfig {
        DependencyConfig::new(
            id.to_string(),
            name.to_string(),
            dependency_type,
            "test-target".to_string(),
            vec!["test-operation".to_string()],
        )
        .unwrap()
    }

    #[test]
    fn test_new_impact_analysis() {
        let analysis = ImpactAnalysis::new();
        assert_eq!(analysis.business_metrics.revenue_weight, 0.3);
        assert_eq!(analysis.risk_factors.availability_weight, 0.3);
    }

    #[test]
    fn test_with_config() {
        let business_metrics = BusinessImpactMetrics {
            revenue_weight: 0.5,
            user_experience_weight: 0.3,
            operational_cost_weight: 0.2,
            security_weight: 0.1,
            compliance_weight: 0.05,
            brand_reputation_weight: 0.05,
        };

        let risk_factors = RiskFactors {
            availability_weight: 0.5,
            performance_weight: 0.3,
            security_weight: 0.2,
            scalability_weight: 0.1,
            maintainability_weight: 0.05,
            compliance_weight: 0.05,
        };

        let analysis = ImpactAnalysis::with_config(business_metrics, risk_factors);
        assert_eq!(analysis.business_metrics.revenue_weight, 0.5);
        assert_eq!(analysis.risk_factors.availability_weight, 0.5);
    }

    #[test]
    fn test_get_base_impact_for_type() {
        let analysis = ImpactAnalysis::new();

        assert_eq!(
            analysis.get_base_impact_for_type(&DependencyType::Infrastructure),
            0.9
        );
        assert_eq!(
            analysis.get_base_impact_for_type(&DependencyType::Security),
            0.9
        );
        assert_eq!(
            analysis.get_base_impact_for_type(&DependencyType::DataFlow),
            0.8
        );
        assert_eq!(
            analysis.get_base_impact_for_type(&DependencyType::ApiCall),
            0.7
        );
    }

    #[test]
    fn test_calculate_critical_functions_impact() {
        let analysis = ImpactAnalysis::new();

        // No critical functions
        assert_eq!(analysis.calculate_critical_functions_impact("test"), 0.3);

        // With critical functions
        let analysis = analysis.add_critical_functions(
            "test".to_string(),
            vec!["func1".to_string(), "func2".to_string()],
        );
        assert_eq!(analysis.calculate_critical_functions_impact("test"), 0.5);
    }

    #[test]
    fn test_default_downtime_costs() {
        let costs = ImpactAnalysis::default_downtime_costs();

        assert_eq!(costs.get(&DependencyType::Infrastructure), Some(&10000.0));
        assert_eq!(costs.get(&DependencyType::Security), Some(&15000.0));
        assert_eq!(costs.get(&DependencyType::DataFlow), Some(&5000.0));
    }
}
