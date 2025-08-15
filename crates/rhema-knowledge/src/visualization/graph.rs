//! Graph visualization and analysis module
//! 
//! This module provides advanced graph analysis and visualization capabilities
//! for knowledge graphs, including centrality analysis, community detection,
//! and graph layout algorithms.

use super::{KnowledgeGraph, KnowledgeNode, KnowledgeEdge};
use crate::types::KnowledgeResult;
use std::collections::{HashMap, HashSet};
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Graph analysis metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetrics {
    /// Number of nodes in the graph
    pub node_count: usize,
    /// Number of edges in the graph
    pub edge_count: usize,
    /// Graph density (edges / possible edges)
    pub density: f64,
    /// Average node degree
    pub average_degree: f64,
    /// Graph diameter (longest shortest path)
    pub diameter: Option<f64>,
    /// Graph radius (minimum eccentricity)
    pub radius: Option<f64>,
    /// Number of connected components
    pub connected_components: usize,
    /// Whether the graph is connected
    pub is_connected: bool,
}

/// Node centrality metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralityMetrics {
    /// Degree centrality for each node
    pub degree_centrality: HashMap<String, f64>,
    /// Betweenness centrality for each node
    pub betweenness_centrality: HashMap<String, f64>,
    /// Closeness centrality for each node
    pub closeness_centrality: HashMap<String, f64>,
    /// Eigenvector centrality for each node
    pub eigenvector_centrality: HashMap<String, f64>,
}

/// Community detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityDetection {
    /// Community assignments for each node
    pub communities: HashMap<String, usize>,
    /// Number of communities detected
    pub community_count: usize,
    /// Modularity score
    pub modularity: f64,
}

/// Graph layout algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutAlgorithm {
    /// Force-directed layout
    ForceDirected,
    /// Circular layout
    Circular,
    /// Hierarchical layout
    Hierarchical,
    /// Random layout
    Random,
    /// Custom layout with fixed positions
    Custom(HashMap<String, (f64, f64)>),
}

/// Graph analyzer for knowledge graphs
pub struct GraphAnalyzer;

impl GraphAnalyzer {
    /// Create a new graph analyzer
    pub fn new() -> Self {
        Self
    }

    /// Calculate basic graph metrics
    pub fn calculate_metrics(&self, graph: &KnowledgeGraph) -> GraphMetrics {
        let node_count = graph.nodes.len();
        let edge_count = graph.edges.len();
        
        // Calculate density
        let possible_edges = if node_count > 1 {
            node_count * (node_count - 1) / 2
        } else {
            0
        };
        let density = if possible_edges > 0 {
            edge_count as f64 / possible_edges as f64
        } else {
            0.0
        };

        // Calculate average degree
        let total_degree = graph.edges.iter()
            .fold(HashMap::new(), |mut acc, edge| {
                *acc.entry(edge.source.clone()).or_insert(0) += 1;
                *acc.entry(edge.target.clone()).or_insert(0) += 1;
                acc
            })
            .values()
            .sum::<usize>();
        let average_degree = if node_count > 0 {
            total_degree as f64 / node_count as f64
        } else {
            0.0
        };

        // Calculate connected components
        let connected_components = self.count_connected_components(graph);
        let is_connected = connected_components == 1 && node_count > 0;

        GraphMetrics {
            node_count,
            edge_count,
            density,
            average_degree,
            diameter: None, // Would need to implement shortest path algorithm
            radius: None,   // Would need to implement shortest path algorithm
            connected_components,
            is_connected,
        }
    }

    /// Calculate centrality metrics for all nodes
    pub fn calculate_centrality(&self, graph: &KnowledgeGraph) -> CentralityMetrics {
        let degree_centrality = self.calculate_degree_centrality(graph);
        let betweenness_centrality = self.calculate_betweenness_centrality(graph);
        let closeness_centrality = self.calculate_closeness_centrality(graph);
        let eigenvector_centrality = self.calculate_eigenvector_centrality(graph);

        CentralityMetrics {
            degree_centrality,
            betweenness_centrality,
            closeness_centrality,
            eigenvector_centrality,
        }
    }

    /// Detect communities in the graph
    pub fn detect_communities(&self, graph: &KnowledgeGraph) -> CommunityDetection {
        // Simple community detection using connected components
        // In a real implementation, this would use more sophisticated algorithms
        let mut communities = HashMap::new();
        let mut visited = HashSet::new();
        let mut community_id = 0;

        for node in &graph.nodes {
            if !visited.contains(&node.id) {
                self.dfs_community_detection(graph, &node.id, community_id, &mut communities, &mut visited);
                community_id += 1;
            }
        }

        CommunityDetection {
            communities,
            community_count: community_id,
            modularity: 0.0, // Would need to implement modularity calculation
        }
    }

    /// Apply layout algorithm to graph
    pub fn apply_layout(&self, graph: &KnowledgeGraph, algorithm: LayoutAlgorithm) -> KnowledgeResult<HashMap<String, (f64, f64)>> {
        match algorithm {
            LayoutAlgorithm::ForceDirected => self.force_directed_layout(graph),
            LayoutAlgorithm::Circular => self.circular_layout(graph),
            LayoutAlgorithm::Hierarchical => self.hierarchical_layout(graph),
            LayoutAlgorithm::Random => self.random_layout(graph),
            LayoutAlgorithm::Custom(positions) => Ok(positions),
        }
    }

    /// Calculate degree centrality for all nodes
    fn calculate_degree_centrality(&self, graph: &KnowledgeGraph) -> HashMap<String, f64> {
        let mut degree_counts = HashMap::new();
        
        // Count degrees
        for edge in &graph.edges {
            *degree_counts.entry(edge.source.clone()).or_insert(0) += 1;
            *degree_counts.entry(edge.target.clone()).or_insert(0) += 1;
        }

        // Normalize by maximum possible degree
        let max_degree = if graph.nodes.len() > 1 {
            graph.nodes.len() - 1
        } else {
            1
        };

        degree_counts.into_iter()
            .map(|(node_id, degree)| {
                (node_id, degree as f64 / max_degree as f64)
            })
            .collect()
    }

    /// Calculate betweenness centrality (simplified version)
    fn calculate_betweenness_centrality(&self, _graph: &KnowledgeGraph) -> HashMap<String, f64> {
        // This is a simplified implementation
        // A full implementation would calculate shortest paths between all pairs
        HashMap::new()
    }

    /// Calculate closeness centrality (simplified version)
    fn calculate_closeness_centrality(&self, _graph: &KnowledgeGraph) -> HashMap<String, f64> {
        // This is a simplified implementation
        // A full implementation would calculate shortest paths from each node
        HashMap::new()
    }

    /// Calculate eigenvector centrality (simplified version)
    fn calculate_eigenvector_centrality(&self, _graph: &KnowledgeGraph) -> HashMap<String, f64> {
        // This is a simplified implementation
        // A full implementation would use power iteration on the adjacency matrix
        HashMap::new()
    }

    /// Count connected components using DFS
    fn count_connected_components(&self, graph: &KnowledgeGraph) -> usize {
        let mut visited = HashSet::new();
        let mut component_count = 0;

        for node in &graph.nodes {
            if !visited.contains(&node.id) {
                self.dfs_traversal(graph, &node.id, &mut visited);
                component_count += 1;
            }
        }

        component_count
    }

    /// DFS traversal for connected components
    fn dfs_traversal(&self, graph: &KnowledgeGraph, node_id: &str, visited: &mut HashSet<String>) {
        visited.insert(node_id.to_string());

        for edge in &graph.edges {
            let neighbor = if edge.source == node_id {
                Some(&edge.target)
            } else if edge.target == node_id {
                Some(&edge.source)
            } else {
                None
            };

            if let Some(neighbor_id) = neighbor {
                if !visited.contains(neighbor_id) {
                    self.dfs_traversal(graph, neighbor_id, visited);
                }
            }
        }
    }

    /// DFS for community detection
    fn dfs_community_detection(
        &self,
        graph: &KnowledgeGraph,
        node_id: &str,
        community_id: usize,
        communities: &mut HashMap<String, usize>,
        visited: &mut HashSet<String>,
    ) {
        visited.insert(node_id.to_string());
        communities.insert(node_id.to_string(), community_id);

        for edge in &graph.edges {
            let neighbor = if edge.source == node_id {
                Some(&edge.target)
            } else if edge.target == node_id {
                Some(&edge.source)
            } else {
                None
            };

            if let Some(neighbor_id) = neighbor {
                if !visited.contains(neighbor_id) {
                    self.dfs_community_detection(graph, neighbor_id, community_id, communities, visited);
                }
            }
        }
    }

    /// Force-directed layout algorithm
    fn force_directed_layout(&self, graph: &KnowledgeGraph) -> KnowledgeResult<HashMap<String, (f64, f64)>> {
        let mut positions = HashMap::new();
        
        // Initialize random positions
        for node in &graph.nodes {
            positions.insert(node.id.clone(), (
                rand::random::<f64>() * 800.0,
                rand::random::<f64>() * 600.0,
            ));
        }

        // Simple force-directed layout (simplified)
        // In a real implementation, this would iterate multiple times
        // and apply repulsive and attractive forces

        Ok(positions)
    }

    /// Circular layout algorithm
    fn circular_layout(&self, graph: &KnowledgeGraph) -> KnowledgeResult<HashMap<String, (f64, f64)>> {
        let mut positions = HashMap::new();
        let center_x = 400.0;
        let center_y = 300.0;
        let radius = 200.0;

        for (i, node) in graph.nodes.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / graph.nodes.len() as f64;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            positions.insert(node.id.clone(), (x, y));
        }

        Ok(positions)
    }

    /// Hierarchical layout algorithm
    fn hierarchical_layout(&self, graph: &KnowledgeGraph) -> KnowledgeResult<HashMap<String, (f64, f64)>> {
        let mut positions = HashMap::new();
        
        // Simple hierarchical layout
        // In a real implementation, this would use proper hierarchical algorithms
        for (i, node) in graph.nodes.iter().enumerate() {
            let level = i / 10; // 10 nodes per level
            let x = 100.0 + (i % 10) as f64 * 60.0;
            let y = 100.0 + level as f64 * 80.0;
            positions.insert(node.id.clone(), (x, y));
        }

        Ok(positions)
    }

    /// Random layout algorithm
    fn random_layout(&self, graph: &KnowledgeGraph) -> KnowledgeResult<HashMap<String, (f64, f64)>> {
        let mut positions = HashMap::new();
        
        for node in &graph.nodes {
            positions.insert(node.id.clone(), (
                rand::random::<f64>() * 800.0,
                rand::random::<f64>() * 600.0,
            ));
        }

        Ok(positions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_metrics_calculation() {
        let analyzer = GraphAnalyzer::new();
        let graph = KnowledgeGraph {
            nodes: vec![
                KnowledgeNode {
                    id: "node1".to_string(),
                    label: "Node 1".to_string(),
                    content_type: crate::types::ContentType::Unknown,
                    weight: 1.0,
                    color: "#FF0000".to_string(),
                    size: 1.0,
                    metadata: HashMap::new(),
                },
                KnowledgeNode {
                    id: "node2".to_string(),
                    label: "Node 2".to_string(),
                    content_type: crate::types::ContentType::Unknown,
                    weight: 1.0,
                    color: "#00FF00".to_string(),
                    size: 1.0,
                    metadata: HashMap::new(),
                },
            ],
            edges: vec![
                KnowledgeEdge {
                    source: "node1".to_string(),
                    target: "node2".to_string(),
                    label: "edge1".to_string(),
                    weight: 1.0,
                    color: "#0000FF".to_string(),
                    thickness: 1.0,
                    metadata: HashMap::new(),
                },
            ],
            metadata: HashMap::new(),
        };

        let metrics = analyzer.calculate_metrics(&graph);
        assert_eq!(metrics.node_count, 2);
        assert_eq!(metrics.edge_count, 1);
        assert_eq!(metrics.density, 1.0);
        assert_eq!(metrics.average_degree, 1.0);
        assert_eq!(metrics.connected_components, 1);
        assert!(metrics.is_connected);
    }

    #[test]
    fn test_community_detection() {
        let analyzer = GraphAnalyzer::new();
        let graph = KnowledgeGraph {
            nodes: vec![
                KnowledgeNode {
                    id: "node1".to_string(),
                    label: "Node 1".to_string(),
                    content_type: crate::types::ContentType::Unknown,
                    weight: 1.0,
                    color: "#FF0000".to_string(),
                    size: 1.0,
                    metadata: HashMap::new(),
                },
                KnowledgeNode {
                    id: "node2".to_string(),
                    label: "Node 2".to_string(),
                    content_type: crate::types::ContentType::Unknown,
                    weight: 1.0,
                    color: "#00FF00".to_string(),
                    size: 1.0,
                    metadata: HashMap::new(),
                },
            ],
            edges: vec![
                KnowledgeEdge {
                    source: "node1".to_string(),
                    target: "node2".to_string(),
                    label: "edge1".to_string(),
                    weight: 1.0,
                    color: "#0000FF".to_string(),
                    thickness: 1.0,
                    metadata: HashMap::new(),
                },
            ],
            metadata: HashMap::new(),
        };

        let communities = analyzer.detect_communities(&graph);
        assert_eq!(communities.community_count, 1);
        assert_eq!(communities.communities.len(), 2);
    }

    #[test]
    fn test_circular_layout() {
        let analyzer = GraphAnalyzer::new();
        let graph = KnowledgeGraph {
            nodes: vec![
                KnowledgeNode {
                    id: "node1".to_string(),
                    label: "Node 1".to_string(),
                    content_type: crate::types::ContentType::Unknown,
                    weight: 1.0,
                    color: "#FF0000".to_string(),
                    size: 1.0,
                    metadata: HashMap::new(),
                },
                KnowledgeNode {
                    id: "node2".to_string(),
                    label: "Node 2".to_string(),
                    content_type: crate::types::ContentType::Unknown,
                    weight: 1.0,
                    color: "#00FF00".to_string(),
                    size: 1.0,
                    metadata: HashMap::new(),
                },
            ],
            edges: vec![],
            metadata: HashMap::new(),
        };

        let positions = analyzer.apply_layout(&graph, LayoutAlgorithm::Circular).unwrap();
        assert_eq!(positions.len(), 2);
    }
}
