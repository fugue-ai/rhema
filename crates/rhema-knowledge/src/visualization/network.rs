//! Network visualization module
//!
//! This module provides network analysis and visualization capabilities
//! for knowledge networks, including network metrics, path analysis,
//! and network layout algorithms.

use super::{KnowledgeEdge, KnowledgeGraph, KnowledgeNode};
use crate::types::KnowledgeResult;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Network metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Number of nodes in the network
    pub node_count: usize,
    /// Number of edges in the network
    pub edge_count: usize,
    /// Network density
    pub density: f64,
    /// Average clustering coefficient
    pub clustering_coefficient: f64,
    /// Average path length
    pub average_path_length: f64,
    /// Network diameter
    pub diameter: f64,
    /// Number of connected components
    pub connected_components: usize,
    /// Whether the network is connected
    pub is_connected: bool,
}

/// Path information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathInfo {
    /// Source node
    pub source: String,
    /// Target node
    pub target: String,
    /// Path length
    pub length: usize,
    /// Path nodes
    pub nodes: Vec<String>,
    /// Path edges
    pub edges: Vec<String>,
}

/// Network analyzer for knowledge networks
pub struct NetworkAnalyzer;

impl NetworkAnalyzer {
    /// Create a new network analyzer
    pub fn new() -> Self {
        Self
    }

    /// Calculate network metrics
    pub fn calculate_metrics(&self, graph: &KnowledgeGraph) -> NetworkMetrics {
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

        // Calculate clustering coefficient
        let clustering_coefficient = self.calculate_clustering_coefficient(graph);

        // Calculate average path length and diameter
        let (average_path_length, diameter) = self.calculate_path_metrics(graph);

        // Calculate connected components
        let connected_components = self.count_connected_components(graph);
        let is_connected = connected_components == 1 && node_count > 0;

        NetworkMetrics {
            node_count,
            edge_count,
            density,
            clustering_coefficient,
            average_path_length,
            diameter,
            connected_components,
            is_connected,
        }
    }

    /// Find shortest path between two nodes
    pub fn find_shortest_path(
        &self,
        graph: &KnowledgeGraph,
        source: &str,
        target: &str,
    ) -> Option<PathInfo> {
        let mut distances = HashMap::<String, usize>::new();
        let mut previous = HashMap::<String, String>::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::<String>::new();

        // Initialize distances
        for node in &graph.nodes {
            distances.insert(node.id.clone(), usize::MAX);
        }
        distances.insert(source.to_string(), 0);
        queue.push_back(source.to_string());

        while let Some(current) = queue.pop_front() {
            if current == target {
                break;
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            // Find neighbors
            for edge in &graph.edges {
                let neighbor = if edge.source == current {
                    Some(&edge.target)
                } else if edge.target == current {
                    Some(&edge.source)
                } else {
                    None
                };

                if let Some(neighbor_id) = neighbor {
                    if !visited.contains(neighbor_id) {
                        let new_distance = distances[&current] + 1;
                        if new_distance < distances[neighbor_id] {
                            distances.insert(neighbor_id.clone(), new_distance);
                            previous.insert(neighbor_id.clone(), current.clone());
                            queue.push_back(neighbor_id.clone());
                        }
                    }
                }
            }
        }

        // Reconstruct path
        if distances[target] == usize::MAX {
            None
        } else {
            let mut path_nodes = Vec::new();
            let mut path_edges = Vec::new();
            let mut current = target.to_string();

            while current != source {
                path_nodes.push(current.clone());
                let prev = previous[&current].clone();

                // Find edge between prev and current
                for edge in &graph.edges {
                    if (edge.source == prev && edge.target == current)
                        || (edge.source == current && edge.target == prev)
                    {
                        path_edges.push(edge.label.clone());
                        break;
                    }
                }

                current = prev;
            }
            path_nodes.push(source.to_string());
            path_nodes.reverse();
            path_edges.reverse();

            Some(PathInfo {
                source: source.to_string(),
                target: target.to_string(),
                length: distances[target],
                nodes: path_nodes,
                edges: path_edges,
            })
        }
    }

    /// Find all paths between two nodes
    pub fn find_all_paths(
        &self,
        graph: &KnowledgeGraph,
        source: &str,
        target: &str,
        max_paths: usize,
    ) -> Vec<PathInfo> {
        let mut paths = Vec::new();
        let mut visited = HashSet::<String>::new();
        let mut current_path = Vec::new();

        self.dfs_all_paths(
            graph,
            source,
            target,
            &mut visited,
            &mut current_path,
            &mut paths,
            max_paths,
        );

        paths
    }

    /// Find nodes within a certain distance
    pub fn find_nodes_within_distance(
        &self,
        graph: &KnowledgeGraph,
        source: &str,
        max_distance: usize,
    ) -> HashMap<String, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::<String>::new();

        distances.insert(source.to_string(), 0);
        queue.push_back((source, 0));

        while let Some((current, distance)) = queue.pop_front() {
            if distance > max_distance {
                continue;
            }

            if visited.contains(current) {
                continue;
            }
            visited.insert(current.to_string());

            // Find neighbors
            for edge in &graph.edges {
                let neighbor = if edge.source == current {
                    Some(&edge.target)
                } else if edge.target == current {
                    Some(&edge.source)
                } else {
                    None
                };

                if let Some(neighbor_id) = neighbor {
                    if !visited.contains(neighbor_id) {
                        let new_distance = distance + 1;
                        distances.insert(neighbor_id.to_string(), new_distance);
                        queue.push_back((neighbor_id, new_distance));
                    }
                }
            }
        }

        distances
    }

    /// Calculate clustering coefficient for the network
    fn calculate_clustering_coefficient(&self, graph: &KnowledgeGraph) -> f64 {
        let mut total_coefficient = 0.0;
        let mut node_count = 0;

        for node in &graph.nodes {
            let neighbors = self.get_neighbors(graph, &node.id);
            if neighbors.len() < 2 {
                continue;
            }

            let mut neighbor_connections = 0;
            for i in 0..neighbors.len() {
                for j in (i + 1)..neighbors.len() {
                    if self.has_edge(graph, &neighbors[i], &neighbors[j]) {
                        neighbor_connections += 1;
                    }
                }
            }

            let possible_connections = neighbors.len() * (neighbors.len() - 1) / 2;
            if possible_connections > 0 {
                total_coefficient += neighbor_connections as f64 / possible_connections as f64;
                node_count += 1;
            }
        }

        if node_count > 0 {
            total_coefficient / node_count as f64
        } else {
            0.0
        }
    }

    /// Calculate path metrics (average path length and diameter)
    fn calculate_path_metrics(&self, graph: &KnowledgeGraph) -> (f64, f64) {
        let mut total_path_length = 0.0;
        let mut path_count = 0;
        let mut max_path_length: f64 = 0.0;

        for i in 0..graph.nodes.len() {
            for j in (i + 1)..graph.nodes.len() {
                if let Some(path) =
                    self.find_shortest_path(graph, &graph.nodes[i].id, &graph.nodes[j].id)
                {
                    total_path_length += path.length as f64;
                    path_count += 1;
                    max_path_length = max_path_length.max(path.length as f64);
                }
            }
        }

        let average_path_length = if path_count > 0 {
            total_path_length / path_count as f64
        } else {
            0.0
        };

        (average_path_length, max_path_length)
    }

    /// Count connected components using DFS
    fn count_connected_components(&self, graph: &KnowledgeGraph) -> usize {
        let mut visited = HashSet::<String>::new();
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

    /// DFS to find all paths between two nodes
    fn dfs_all_paths(
        &self,
        graph: &KnowledgeGraph,
        current: &str,
        target: &str,
        visited: &mut HashSet<String>,
        current_path: &mut Vec<String>,
        paths: &mut Vec<PathInfo>,
        max_paths: usize,
    ) {
        if paths.len() >= max_paths {
            return;
        }

        current_path.push(current.to_string());

        if current == target {
            let mut path_edges = Vec::new();
            for i in 0..current_path.len() - 1 {
                for edge in &graph.edges {
                    if (edge.source == current_path[i] && edge.target == current_path[i + 1])
                        || (edge.source == current_path[i + 1] && edge.target == current_path[i])
                    {
                        path_edges.push(edge.label.clone());
                        break;
                    }
                }
            }

            paths.push(PathInfo {
                source: current_path[0].clone(),
                target: target.to_string(),
                length: current_path.len() - 1,
                nodes: current_path.clone(),
                edges: path_edges,
            });
        } else {
            visited.insert(current.to_string());

            for edge in &graph.edges {
                let neighbor = if edge.source == current {
                    Some(&edge.target)
                } else if edge.target == current {
                    Some(&edge.source)
                } else {
                    None
                };

                if let Some(neighbor_id) = neighbor {
                    if !visited.contains(neighbor_id) {
                        self.dfs_all_paths(
                            graph,
                            neighbor_id,
                            target,
                            visited,
                            current_path,
                            paths,
                            max_paths,
                        );
                    }
                }
            }

            visited.remove(current);
        }

        current_path.pop();
    }

    /// Get neighbors of a node
    fn get_neighbors(&self, graph: &KnowledgeGraph, node_id: &str) -> Vec<String> {
        let mut neighbors = Vec::new();

        for edge in &graph.edges {
            if edge.source == node_id {
                neighbors.push(edge.target.clone());
            } else if edge.target == node_id {
                neighbors.push(edge.source.clone());
            }
        }

        neighbors
    }

    /// Check if there's an edge between two nodes
    fn has_edge(&self, graph: &KnowledgeGraph, node1: &str, node2: &str) -> bool {
        graph.edges.iter().any(|edge| {
            (edge.source == node1 && edge.target == node2)
                || (edge.source == node2 && edge.target == node1)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_metrics_calculation() {
        let analyzer = NetworkAnalyzer::new();
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
                KnowledgeNode {
                    id: "node3".to_string(),
                    label: "Node 3".to_string(),
                    content_type: crate::types::ContentType::Unknown,
                    weight: 1.0,
                    color: "#0000FF".to_string(),
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
                    color: "#000000".to_string(),
                    thickness: 1.0,
                    metadata: HashMap::new(),
                },
                KnowledgeEdge {
                    source: "node2".to_string(),
                    target: "node3".to_string(),
                    label: "edge2".to_string(),
                    weight: 1.0,
                    color: "#000000".to_string(),
                    thickness: 1.0,
                    metadata: HashMap::new(),
                },
            ],
            metadata: HashMap::new(),
        };

        let metrics = analyzer.calculate_metrics(&graph);
        assert_eq!(metrics.node_count, 3);
        assert_eq!(metrics.edge_count, 2);
        assert_eq!(metrics.connected_components, 1);
        assert!(metrics.is_connected);
    }

    #[test]
    fn test_shortest_path_finding() {
        let analyzer = NetworkAnalyzer::new();
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
                KnowledgeNode {
                    id: "node3".to_string(),
                    label: "Node 3".to_string(),
                    content_type: crate::types::ContentType::Unknown,
                    weight: 1.0,
                    color: "#0000FF".to_string(),
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
                    color: "#000000".to_string(),
                    thickness: 1.0,
                    metadata: HashMap::new(),
                },
                KnowledgeEdge {
                    source: "node2".to_string(),
                    target: "node3".to_string(),
                    label: "edge2".to_string(),
                    weight: 1.0,
                    color: "#000000".to_string(),
                    thickness: 1.0,
                    metadata: HashMap::new(),
                },
            ],
            metadata: HashMap::new(),
        };

        let path = analyzer.find_shortest_path(&graph, "node1", "node3");
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.length, 2);
        assert_eq!(path.nodes, vec!["node1", "node2", "node3"]);
    }

    #[test]
    fn test_nodes_within_distance() {
        let analyzer = NetworkAnalyzer::new();
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
                KnowledgeNode {
                    id: "node3".to_string(),
                    label: "Node 3".to_string(),
                    content_type: crate::types::ContentType::Unknown,
                    weight: 1.0,
                    color: "#0000FF".to_string(),
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
                    color: "#000000".to_string(),
                    thickness: 1.0,
                    metadata: HashMap::new(),
                },
                KnowledgeEdge {
                    source: "node2".to_string(),
                    target: "node3".to_string(),
                    label: "edge2".to_string(),
                    weight: 1.0,
                    color: "#000000".to_string(),
                    thickness: 1.0,
                    metadata: HashMap::new(),
                },
            ],
            metadata: HashMap::new(),
        };

        let distances = analyzer.find_nodes_within_distance(&graph, "node1", 2);
        assert_eq!(distances.len(), 3);
        assert_eq!(distances["node1"], 0);
        assert_eq!(distances["node2"], 1);
        assert_eq!(distances["node3"], 2);
    }
}
