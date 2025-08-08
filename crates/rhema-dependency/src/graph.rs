use chrono::{DateTime, Utc};
use petgraph::algo::is_cyclic_directed;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{Dfs, EdgeRef};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::error::{Error, Result};
use crate::types::{DependencyConfig, DependencyType, HealthStatus, ImpactScore};

/// Node in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    /// Unique identifier
    pub id: String,
    /// Dependency configuration
    pub config: DependencyConfig,
    /// Current health status
    pub health_status: HealthStatus,
    /// Current health metrics
    pub health_metrics: Option<ImpactScore>,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}

/// Edge in the dependency graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    /// Source dependency ID
    pub source: String,
    /// Target dependency ID
    pub target: String,
    /// Type of dependency relationship
    pub relationship_type: String,
    /// Strength of the dependency (0.0 to 1.0)
    pub strength: f64,
    /// Operations that can be performed
    pub operations: Vec<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}

/// Dependency graph manager
pub struct DependencyGraph {
    /// The underlying graph structure
    graph: DiGraph<DependencyNode, DependencyEdge>,
    /// Mapping from dependency ID to node index
    node_indices: HashMap<String, NodeIndex>,
    /// Mapping from node index to dependency ID
    reverse_indices: HashMap<NodeIndex, String>,
}

impl DependencyGraph {
    /// Create a new empty dependency graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_indices: HashMap::new(),
            reverse_indices: HashMap::new(),
        }
    }

    /// Add a dependency node to the graph
    pub fn add_node(&mut self, config: DependencyConfig) -> Result<()> {
        if self.node_indices.contains_key(&config.id) {
            return Err(Error::DependencyAlreadyExists(config.id.clone()));
        }

        let id = config.id.clone();
        let node = DependencyNode {
            id: id.clone(),
            config,
            health_status: HealthStatus::Unknown,
            health_metrics: None,
            last_updated: Utc::now(),
        };

        let node_index = self.graph.add_node(node);
        self.node_indices.insert(id.clone(), node_index);
        self.reverse_indices.insert(node_index, id);

        Ok(())
    }

    /// Remove a dependency node from the graph
    pub fn remove_node(&mut self, dependency_id: &str) -> Result<()> {
        let node_index = self
            .node_indices
            .get(dependency_id)
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))?
            .clone();

        // Remove all edges connected to this node
        let edges_to_remove: Vec<_> = self.graph.edges(node_index).map(|edge| edge.id()).collect();

        for edge_id in edges_to_remove {
            self.graph.remove_edge(edge_id);
        }

        // Remove the node
        self.graph.remove_node(node_index);
        self.node_indices.remove(dependency_id);
        self.reverse_indices.remove(&node_index);

        Ok(())
    }

    /// Add a dependency edge between two nodes
    pub fn add_edge(
        &mut self,
        source_id: &str,
        target_id: &str,
        relationship_type: String,
        strength: f64,
        operations: Vec<String>,
    ) -> Result<()> {
        // Validate strength is between 0.0 and 1.0
        if !(0.0..=1.0).contains(&strength) {
            return Err(Error::Validation(
                "Dependency strength must be between 0.0 and 1.0".to_string(),
            ));
        }

        let source_index = self
            .node_indices
            .get(source_id)
            .ok_or_else(|| Error::DependencyNotFound(source_id.to_string()))?;

        let target_index = self
            .node_indices
            .get(target_id)
            .ok_or_else(|| Error::DependencyNotFound(target_id.to_string()))?;

        let edge = DependencyEdge {
            source: source_id.to_string(),
            target: target_id.to_string(),
            relationship_type,
            strength,
            operations,
            created_at: Utc::now(),
        };

        self.graph.add_edge(*source_index, *target_index, edge);

        // Check for circular dependencies
        if self.has_circular_dependencies()? {
            // Remove the edge we just added
            self.graph
                .remove_edge(self.graph.find_edge(*source_index, *target_index).unwrap());
            return Err(Error::CircularDependency(format!(
                "Adding edge from {} to {} would create a circular dependency",
                source_id, target_id
            )));
        }

        Ok(())
    }

    /// Remove a dependency edge
    pub fn remove_edge(&mut self, source_id: &str, target_id: &str) -> Result<()> {
        let source_index = self
            .node_indices
            .get(source_id)
            .ok_or_else(|| Error::DependencyNotFound(source_id.to_string()))?;

        let target_index = self
            .node_indices
            .get(target_id)
            .ok_or_else(|| Error::DependencyNotFound(target_id.to_string()))?;

        let edge_index = self
            .graph
            .find_edge(*source_index, *target_index)
            .ok_or_else(|| Error::GraphOperation("Edge not found".to_string()))?;

        self.graph.remove_edge(edge_index);
        Ok(())
    }

    /// Check if the graph has circular dependencies
    pub fn has_circular_dependencies(&self) -> Result<bool> {
        Ok(is_cyclic_directed(&self.graph))
    }

    /// Find all circular dependencies
    pub fn find_circular_dependencies(&self) -> Result<Vec<Vec<String>>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node_index in self.graph.node_indices() {
            if !visited.contains(&node_index) {
                let mut path = Vec::new();
                if self.dfs_cycle_detection(node_index, &mut visited, &mut rec_stack, &mut path)? {
                    cycles.push(path);
                }
            }
        }

        Ok(cycles)
    }

    /// DFS-based cycle detection
    fn dfs_cycle_detection(
        &self,
        node_index: NodeIndex,
        visited: &mut HashSet<NodeIndex>,
        rec_stack: &mut HashSet<NodeIndex>,
        path: &mut Vec<String>,
    ) -> Result<bool> {
        visited.insert(node_index);
        rec_stack.insert(node_index);

        let dependency_id = self.reverse_indices.get(&node_index).ok_or_else(|| {
            Error::GraphOperation("Node index not found in reverse mapping".to_string())
        })?;

        path.push(dependency_id.clone());

        for neighbor in self.graph.neighbors(node_index) {
            if !visited.contains(&neighbor) {
                if self.dfs_cycle_detection(neighbor, visited, rec_stack, path)? {
                    return Ok(true);
                }
            } else if rec_stack.contains(&neighbor) {
                // Found a cycle
                return Ok(true);
            }
        }

        rec_stack.remove(&node_index);
        path.pop();
        Ok(false)
    }

    /// Get all dependencies that depend on a given dependency
    pub fn get_dependents(&self, dependency_id: &str) -> Result<Vec<String>> {
        let node_index = self
            .node_indices
            .get(dependency_id)
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))?;

        let dependents: Vec<String> = self
            .graph
            .neighbors_directed(*node_index, petgraph::Direction::Outgoing)
            .filter_map(|idx| self.reverse_indices.get(&idx).cloned())
            .collect();

        Ok(dependents)
    }

    /// Get all dependencies that a given dependency depends on
    pub fn get_dependencies(&self, dependency_id: &str) -> Result<Vec<String>> {
        let node_index = self
            .node_indices
            .get(dependency_id)
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))?;

        let dependencies: Vec<String> = self
            .graph
            .neighbors_directed(*node_index, petgraph::Direction::Incoming)
            .filter_map(|idx| self.reverse_indices.get(&idx).cloned())
            .collect();

        Ok(dependencies)
    }

    /// Get all dependencies of a specific type
    pub fn get_dependencies_by_type(&self, dependency_type: DependencyType) -> Result<Vec<String>> {
        let dependencies: Vec<String> = self
            .graph
            .node_indices()
            .filter_map(|idx| {
                let node = &self.graph[idx];
                if node.config.dependency_type == dependency_type {
                    Some(node.id.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(dependencies)
    }

    /// Get all dependencies with a specific health status
    pub fn get_dependencies_by_health(&self, health_status: HealthStatus) -> Result<Vec<String>> {
        let dependencies: Vec<String> = self
            .graph
            .node_indices()
            .filter_map(|idx| {
                let node = &self.graph[idx];
                if node.health_status == health_status {
                    Some(node.id.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(dependencies)
    }

    /// Update the health status of a dependency
    pub fn update_health_status(
        &mut self,
        dependency_id: &str,
        health_status: HealthStatus,
    ) -> Result<()> {
        let node_index = self
            .node_indices
            .get(dependency_id)
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))?;

        let node = &mut self.graph[*node_index];
        node.health_status = health_status;
        node.last_updated = Utc::now();

        Ok(())
    }

    /// Update the health metrics of a dependency
    pub fn update_health_metrics(
        &mut self,
        dependency_id: &str,
        health_metrics: ImpactScore,
    ) -> Result<()> {
        let node_index = self
            .node_indices
            .get(dependency_id)
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))?;

        let node = &mut self.graph[*node_index];
        node.health_metrics = Some(health_metrics);
        node.last_updated = Utc::now();

        Ok(())
    }

    /// Get the dependency configuration
    pub fn get_dependency_config(&self, dependency_id: &str) -> Result<&DependencyConfig> {
        let node_index = self
            .node_indices
            .get(dependency_id)
            .ok_or_else(|| Error::DependencyNotFound(dependency_id.to_string()))?;

        Ok(&self.graph[*node_index].config)
    }

    /// Get all dependency configurations
    pub fn get_all_dependency_configs(&self) -> Vec<&DependencyConfig> {
        self.graph
            .node_indices()
            .map(|idx| &self.graph[idx].config)
            .collect()
    }

    /// Get the number of nodes in the graph
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of edges in the graph
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Check if the graph is empty
    pub fn is_empty(&self) -> bool {
        self.graph.node_count() == 0
    }

    /// Get all dependency IDs
    pub fn get_all_dependency_ids(&self) -> Vec<String> {
        self.graph
            .node_indices()
            .filter_map(|idx| self.reverse_indices.get(&idx).cloned())
            .collect()
    }

    /// Get the dependency graph as a DOT format string for visualization
    pub fn to_dot(&self) -> String {
        use std::fmt::Write;

        let mut dot = String::new();
        writeln!(&mut dot, "digraph DependencyGraph {{").unwrap();
        writeln!(&mut dot, "  rankdir=TB;").unwrap();
        writeln!(&mut dot, "  node [shape=box, style=filled];").unwrap();

        // Add nodes
        for node_index in self.graph.node_indices() {
            let node = &self.graph[node_index];
            let color = match node.health_status {
                HealthStatus::Healthy => "lightgreen",
                HealthStatus::Degraded => "yellow",
                HealthStatus::Unhealthy => "orange",
                HealthStatus::Down => "red",
                HealthStatus::Unknown => "gray",
            };
            writeln!(
                &mut dot,
                "  \"{}\" [label=\"{}\", fillcolor={}];",
                node.id, node.config.name, color
            )
            .unwrap();
        }

        // Add edges
        for edge in self.graph.edge_indices() {
            let (source, target) = self.graph.edge_endpoints(edge).unwrap();
            let edge_data = &self.graph[edge];
            writeln!(
                &mut dot,
                "  \"{}\" -> \"{}\" [label=\"{}\"];",
                edge_data.source, edge_data.target, edge_data.relationship_type
            )
            .unwrap();
        }

        writeln!(&mut dot, "}}").unwrap();
        dot
    }
}

impl Default for DependencyGraph {
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
    fn test_new_graph() {
        let graph = DependencyGraph::new();
        assert!(graph.is_empty());
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = DependencyGraph::new();
        let config = create_test_config("test-1", "Test 1", DependencyType::ApiCall);

        assert!(graph.add_node(config).is_ok());
        assert_eq!(graph.node_count(), 1);
        assert!(!graph.is_empty());
    }

    #[test]
    fn test_add_duplicate_node() {
        let mut graph = DependencyGraph::new();
        let config1 = create_test_config("test-1", "Test 1", DependencyType::ApiCall);
        let config2 = create_test_config("test-1", "Test 2", DependencyType::DataFlow);

        assert!(graph.add_node(config1).is_ok());
        assert!(graph.add_node(config2).is_err());
    }

    #[test]
    fn test_add_edge() {
        let mut graph = DependencyGraph::new();
        let config1 = create_test_config("test-1", "Test 1", DependencyType::ApiCall);
        let config2 = create_test_config("test-2", "Test 2", DependencyType::DataFlow);

        graph.add_node(config1).unwrap();
        graph.add_node(config2).unwrap();

        assert!(graph
            .add_edge(
                "test-1",
                "test-2",
                "depends_on".to_string(),
                0.8,
                vec!["read".to_string()]
            )
            .is_ok());
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut graph = DependencyGraph::new();
        let config1 = create_test_config("test-1", "Test 1", DependencyType::ApiCall);
        let config2 = create_test_config("test-2", "Test 2", DependencyType::DataFlow);
        let config3 = create_test_config("test-3", "Test 3", DependencyType::Infrastructure);

        graph.add_node(config1).unwrap();
        graph.add_node(config2).unwrap();
        graph.add_node(config3).unwrap();

        // Create a cycle: test-1 -> test-2 -> test-3 -> test-1
        graph
            .add_edge(
                "test-1",
                "test-2",
                "depends_on".to_string(),
                0.8,
                vec!["read".to_string()],
            )
            .unwrap();
        graph
            .add_edge(
                "test-2",
                "test-3",
                "depends_on".to_string(),
                0.8,
                vec!["read".to_string()],
            )
            .unwrap();

        // This should fail due to circular dependency
        assert!(graph
            .add_edge(
                "test-3",
                "test-1",
                "depends_on".to_string(),
                0.8,
                vec!["read".to_string()]
            )
            .is_err());
    }

    #[test]
    fn test_get_dependents() {
        let mut graph = DependencyGraph::new();
        let config1 = create_test_config("test-1", "Test 1", DependencyType::ApiCall);
        let config2 = create_test_config("test-2", "Test 2", DependencyType::DataFlow);
        let config3 = create_test_config("test-3", "Test 3", DependencyType::Infrastructure);

        graph.add_node(config1).unwrap();
        graph.add_node(config2).unwrap();
        graph.add_node(config3).unwrap();

        graph
            .add_edge(
                "test-1",
                "test-2",
                "depends_on".to_string(),
                0.8,
                vec!["read".to_string()],
            )
            .unwrap();
        graph
            .add_edge(
                "test-1",
                "test-3",
                "depends_on".to_string(),
                0.8,
                vec!["read".to_string()],
            )
            .unwrap();

        let dependents = graph.get_dependents("test-1").unwrap();
        assert_eq!(dependents.len(), 2);
        assert!(dependents.contains(&"test-2".to_string()));
        assert!(dependents.contains(&"test-3".to_string()));
    }

    #[test]
    fn test_update_health_status() {
        let mut graph = DependencyGraph::new();
        let config = create_test_config("test-1", "Test 1", DependencyType::ApiCall);

        graph.add_node(config).unwrap();
        graph
            .update_health_status("test-1", HealthStatus::Healthy)
            .unwrap();

        let node_index = graph.node_indices.get("test-1").unwrap();
        assert_eq!(
            graph.graph[*node_index].health_status,
            HealthStatus::Healthy
        );
    }

    #[test]
    fn test_to_dot() {
        let mut graph = DependencyGraph::new();
        let config1 = create_test_config("test-1", "Test 1", DependencyType::ApiCall);
        let config2 = create_test_config("test-2", "Test 2", DependencyType::DataFlow);

        graph.add_node(config1).unwrap();
        graph.add_node(config2).unwrap();
        graph
            .add_edge(
                "test-1",
                "test-2",
                "depends_on".to_string(),
                0.8,
                vec!["read".to_string()],
            )
            .unwrap();

        let dot = graph.to_dot();
        assert!(dot.contains("digraph DependencyGraph"));
        assert!(dot.contains("test-1"));
        assert!(dot.contains("test-2"));
        assert!(dot.contains("depends_on"));
    }
}
