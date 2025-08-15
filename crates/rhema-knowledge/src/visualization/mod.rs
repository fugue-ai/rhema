//! Knowledge visualization module for Rhema
//! 
//! This module provides capabilities to visualize knowledge relationships,
//! networks, patterns, and insights from the knowledge system.

use crate::types::{ContentType, KnowledgeResult, SemanticResult};
use crate::temporal::types::{ContentAccess, TemporalRelationshipType};
use crate::TemporalContextRelationship;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Datelike, Timelike, Utc};

pub mod graph;
pub mod charts;
pub mod network;
pub mod patterns;

/// Configuration for knowledge visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    /// Maximum number of nodes to display in graphs
    pub max_nodes: usize,
    /// Maximum number of edges to display in graphs
    pub max_edges: usize,
    /// Whether to include temporal information in visualizations
    pub include_temporal: bool,
    /// Whether to include semantic relationships in visualizations
    pub include_semantic: bool,
    /// Output format for visualizations
    pub output_format: VisualizationFormat,
    /// Color scheme for visualizations
    pub color_scheme: ColorScheme,
}

/// Supported visualization output formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualizationFormat {
    /// JSON format for programmatic access
    Json,
    /// SVG format for web display
    Svg,
    /// PNG format for static images
    Png,
    /// HTML format with interactive features
    Html,
    /// DOT format for Graphviz
    Dot,
}

/// Color schemes for visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorScheme {
    /// Default color scheme
    Default,
    /// High contrast color scheme
    HighContrast,
    /// Colorblind-friendly color scheme
    ColorblindFriendly,
    /// Custom color scheme
    Custom(HashMap<String, String>),
}

/// Node in a knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    /// Unique identifier for the node
    pub id: String,
    /// Label/name for the node
    pub label: String,
    /// Type of knowledge content
    pub content_type: ContentType,
    /// Node weight/importance
    pub weight: f64,
    /// Node color (hex string)
    pub color: String,
    /// Node size
    pub size: f64,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Edge in a knowledge graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    /// Source node ID
    pub source: String,
    /// Target node ID
    pub target: String,
    /// Edge label/type
    pub label: String,
    /// Edge weight/strength
    pub weight: f64,
    /// Edge color (hex string)
    pub color: String,
    /// Edge thickness
    pub thickness: f64,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Knowledge graph structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    /// Graph nodes
    pub nodes: Vec<KnowledgeNode>,
    /// Graph edges
    pub edges: Vec<KnowledgeEdge>,
    /// Graph metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Knowledge visualization engine
pub struct KnowledgeVisualizer {
    config: VisualizationConfig,
}

impl KnowledgeVisualizer {
    /// Create a new knowledge visualizer with the given configuration
    pub fn new(config: VisualizationConfig) -> Self {
        Self { config }
    }

    /// Create a default knowledge visualizer
    pub fn default() -> Self {
        Self {
            config: VisualizationConfig {
                max_nodes: 100,
                max_edges: 200,
                include_temporal: true,
                include_semantic: true,
                output_format: VisualizationFormat::Json,
                color_scheme: ColorScheme::Default,
            },
        }
    }

    /// Generate a knowledge relationship graph
    pub async fn generate_relationship_graph(
        &self,
        content_ids: &[String],
        semantic_results: &[SemanticResult],
        temporal_relationships: &[(String, TemporalContextRelationship)], // (source_id, relationship)
    ) -> KnowledgeResult<KnowledgeGraph> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_map = HashMap::new();

        // Add nodes for content
        for (i, content_id) in content_ids.iter().enumerate() {
            if i >= self.config.max_nodes {
                break;
            }

            let node = KnowledgeNode {
                id: content_id.clone(),
                label: content_id.clone(),
                content_type: ContentType::Unknown, // Default, should be extracted from actual content
                weight: 1.0,
                color: self.get_node_color(i),
                size: 1.0,
                metadata: HashMap::new(),
            };

            node_map.insert(content_id.clone(), i);
            nodes.push(node);
        }

        // Add edges for semantic relationships
        if self.config.include_semantic {
            for result in semantic_results {
                if let (Some(source_idx), Some(target_idx)) = (
                    node_map.get(&result.cache_key),
                    node_map.get(&result.content),
                ) {
                    if edges.len() >= self.config.max_edges {
                        break;
                    }

                    let edge = KnowledgeEdge {
                        source: content_ids[*source_idx].clone(),
                        target: content_ids[*target_idx].clone(),
                        label: "semantic".to_string(),
                        weight: result.relevance_score as f64,
                        color: "#4CAF50".to_string(), // Green for semantic relationships
                        thickness: result.relevance_score as f64,
                        metadata: HashMap::new(),
                    };

                    edges.push(edge);
                }
            }
        }

        // Add edges for temporal relationships
        if self.config.include_temporal {
            for (source_content_id, relationship) in temporal_relationships {
                if let (Some(source_idx), Some(target_idx)) = (
                    node_map.get(source_content_id),
                    node_map.get(&relationship.target_content_id),
                ) {
                    if edges.len() >= self.config.max_edges {
                        break;
                    }

                    let edge = KnowledgeEdge {
                        source: content_ids[*source_idx].clone(),
                        target: content_ids[*target_idx].clone(),
                        label: format!("{:?}", relationship.relationship_type),
                        weight: relationship.confidence,
                        color: "#2196F3".to_string(), // Blue for temporal relationships
                        thickness: relationship.confidence,
                        metadata: HashMap::new(),
                    };

                    edges.push(edge);
                }
            }
        }

        Ok(KnowledgeGraph {
            nodes,
            edges,
            metadata: HashMap::new(),
        })
    }

    /// Generate a temporal pattern visualization
    pub async fn generate_temporal_pattern(
        &self,
        content_access: &[ContentAccess],
    ) -> KnowledgeResult<TemporalPatternVisualization> {
        let mut hourly_patterns = HashMap::new();
        let mut daily_patterns = HashMap::new();
        let mut weekly_patterns = HashMap::new();

        for access in content_access {
            let hour = access.access_time.hour();
            let day = access.access_time.weekday().num_days_from_monday();
            let week = access.access_time.iso_week().week();

            *hourly_patterns.entry(hour).or_insert(0) += 1;
            *daily_patterns.entry(day).or_insert(0) += 1;
            *weekly_patterns.entry(week).or_insert(0) += 1;
        }

        Ok(TemporalPatternVisualization {
            hourly_patterns,
            daily_patterns,
            weekly_patterns,
            metadata: HashMap::new(),
        })
    }

    /// Generate a content type distribution chart
    pub async fn generate_content_type_distribution(
        &self,
        content_types: &[(String, ContentType)],
    ) -> KnowledgeResult<ContentTypeDistribution> {
        let mut distribution = HashMap::new();

        for (_, content_type) in content_types {
            *distribution.entry(content_type.clone()).or_insert(0) += 1;
        }

        Ok(ContentTypeDistribution {
            distribution,
            metadata: HashMap::new(),
        })
    }

    /// Export visualization to the configured format
    pub async fn export_visualization(
        &self,
        graph: &KnowledgeGraph,
    ) -> KnowledgeResult<String> {
        match self.config.output_format {
            VisualizationFormat::Json => self.export_as_json(graph),
            VisualizationFormat::Svg => self.export_as_svg(graph),
            VisualizationFormat::Html => self.export_as_html(graph),
            VisualizationFormat::Dot => self.export_as_dot(graph),
            VisualizationFormat::Png => self.export_as_png(graph),
        }
    }

    /// Get node color based on index and color scheme
    fn get_node_color(&self, index: usize) -> String {
        match &self.config.color_scheme {
            ColorScheme::Default => {
                let colors = vec![
                    "#FF6B6B", "#4ECDC4", "#45B7D1", "#96CEB4", "#FFEAA7",
                    "#DDA0DD", "#98D8C8", "#F7DC6F", "#BB8FCE", "#85C1E9",
                ];
                colors[index % colors.len()].to_string()
            }
            ColorScheme::HighContrast => {
                let colors = vec!["#000000", "#FFFFFF", "#FF0000", "#00FF00", "#0000FF"];
                colors[index % colors.len()].to_string()
            }
            ColorScheme::ColorblindFriendly => {
                let colors = vec![
                    "#E69F00", "#56B4E9", "#009E73", "#F0E442", "#0072B2",
                    "#D55E00", "#CC79A7", "#999999",
                ];
                colors[index % colors.len()].to_string()
            }
            ColorScheme::Custom(colors) => {
                colors.get(&index.to_string())
                    .cloned()
                    .unwrap_or_else(|| "#000000".to_string())
            }
        }
    }

    /// Export graph as JSON
    fn export_as_json(&self, graph: &KnowledgeGraph) -> KnowledgeResult<String> {
        serde_json::to_string_pretty(graph)
            .map_err(|e| crate::types::KnowledgeError::SerdeJsonError(e))
    }

    /// Export graph as SVG
    fn export_as_svg(&self, graph: &KnowledgeGraph) -> KnowledgeResult<String> {
        // Simple SVG generation - in a real implementation, this would be more sophisticated
        let mut svg = String::from(
            r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">"#,
        );

        // Add nodes
        for node in &graph.nodes {
            let x = 100.0 + (node.weight * 50.0);
            let y = 100.0 + (node.weight * 50.0);
            svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="black" stroke-width="2"/>"#,
                x, y, node.size * 10.0, node.color
            ));
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle" font-size="12">{}</text>"#,
                x, y + 5.0, node.label
            ));
        }

        // Add edges
        for edge in &graph.edges {
            svg.push_str(&format!(
                r#"<line x1="100" y1="100" x2="200" y2="200" stroke="{}" stroke-width="{}"/>"#,
                edge.color, edge.thickness * 2.0
            ));
        }

        svg.push_str("</svg>");
        Ok(svg)
    }

    /// Export graph as HTML
    fn export_as_html(&self, graph: &KnowledgeGraph) -> KnowledgeResult<String> {
        let json_data = self.export_as_json(graph)?;
        let html = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <title>Knowledge Graph Visualization</title>
    <script src="https://d3js.org/d3.v7.min.js"></script>
    <style>
        .node {{ cursor: pointer; }}
        .link {{ stroke: #999; stroke-opacity: 0.6; }}
    </style>
</head>
<body>
    <div id="graph"></div>
    <script>
        const data = {};
        // D3.js visualization code would go here
        console.log('Knowledge graph data:', data);
    </script>
</body>
</html>
"#,
            json_data
        );
        Ok(html)
    }

    /// Export graph as DOT format
    fn export_as_dot(&self, graph: &KnowledgeGraph) -> KnowledgeResult<String> {
        let mut dot = String::from("digraph KnowledgeGraph {\n");

        // Add nodes
        for node in &graph.nodes {
            dot.push_str(&format!(
                r#"    "{}" [label="{}", color="{}", style=filled];"#,
                node.id, node.label, node.color
            ));
            dot.push('\n');
        }

        // Add edges
        for edge in &graph.edges {
            dot.push_str(&format!(
                r#"    "{}" -> "{}" [label="{}", color="{}"];"#,
                edge.source, edge.target, edge.label, edge.color
            ));
            dot.push('\n');
        }

        dot.push_str("}\n");
        Ok(dot)
    }

    /// Export graph as PNG (placeholder)
    fn export_as_png(&self, _graph: &KnowledgeGraph) -> KnowledgeResult<String> {
        // In a real implementation, this would use a library like plotters or image
        Err(crate::types::KnowledgeError::ConfigurationError(
            "PNG export not yet implemented".to_string(),
        ))
    }
}

/// Temporal pattern visualization data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPatternVisualization {
    /// Hourly access patterns
    pub hourly_patterns: HashMap<u32, usize>,
    /// Daily access patterns
    pub daily_patterns: HashMap<u32, usize>,
    /// Weekly access patterns
    pub weekly_patterns: HashMap<u32, usize>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Content type distribution data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentTypeDistribution {
    /// Distribution of content types
    pub distribution: HashMap<ContentType, usize>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::temporal::types::TemporalRelationshipType;
    use crate::TemporalContextRelationship;

    #[tokio::test]
    async fn test_visualizer_creation() {
        let config = VisualizationConfig {
            max_nodes: 50,
            max_edges: 100,
            include_temporal: true,
            include_semantic: true,
            output_format: VisualizationFormat::Json,
            color_scheme: ColorScheme::Default,
        };

        let visualizer = KnowledgeVisualizer::new(config);
        assert_eq!(visualizer.config.max_nodes, 50);
    }

    #[tokio::test]
    async fn test_relationship_graph_generation() {
        let visualizer = KnowledgeVisualizer::default();
        let content_ids = vec!["content1".to_string(), "content2".to_string()];
        let semantic_results = vec![
            SemanticResult {
                cache_key: "content1".to_string(),
                content: "content1".to_string(),
                embedding: vec![0.1, 0.2, 0.3],
                relevance_score: 0.8,
                semantic_tags: vec!["tag1".to_string()],
                metadata: crate::types::SearchResultMetadata::default(),
                cache_info: None,
            },
        ];
        let temporal_relationships = vec![
            (
                "content1".to_string(),
                TemporalContextRelationship {
                    relationship_type: TemporalRelationshipType::Sequential {
                        order: 1,
                        gap_duration: std::time::Duration::from_secs(3600),
                    },
                    target_content_id: "content2".to_string(),
                    confidence: 0.8,
                    temporal_distance: std::time::Duration::from_secs(3600),
                    relevance_score: 0.8,
                },
            ),
        ];

        let graph = visualizer
            .generate_relationship_graph(&content_ids, &semantic_results, &temporal_relationships)
            .await
            .unwrap();

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 2); // One semantic edge + one temporal edge
    }

    #[tokio::test]
    async fn test_json_export() {
        let visualizer = KnowledgeVisualizer::default();
        let graph = KnowledgeGraph {
            nodes: vec![KnowledgeNode {
                id: "test".to_string(),
                label: "Test".to_string(),
                content_type: ContentType::Unknown,
                weight: 1.0,
                color: "#FF0000".to_string(),
                size: 1.0,
                metadata: HashMap::new(),
            }],
            edges: vec![],
            metadata: HashMap::new(),
        };

        let json = visualizer.export_visualization(&graph).await.unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("Test"));
    }
}
