//! Chart visualization module
//!
//! This module provides various chart types for visualizing knowledge data,
//! including bar charts, line charts, pie charts, and scatter plots.

use crate::types::KnowledgeResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Chart type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChartType {
    /// Bar chart
    Bar,
    /// Line chart
    Line,
    /// Pie chart
    Pie,
    /// Scatter plot
    Scatter,
    /// Area chart
    Area,
    /// Histogram
    Histogram,
    /// Heatmap
    Heatmap,
}

/// Chart data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    /// X-axis value
    pub x: f64,
    /// Y-axis value
    pub y: f64,
    /// Data point label
    pub label: Option<String>,
    /// Data point color
    pub color: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Chart dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartDataset {
    /// Dataset label
    pub label: String,
    /// Data points
    pub data: Vec<DataPoint>,
    /// Dataset color
    pub color: String,
    /// Whether to show this dataset
    pub visible: bool,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
    /// Chart title
    pub title: String,
    /// X-axis label
    pub x_label: String,
    /// Y-axis label
    pub y_label: String,
    /// Chart width
    pub width: u32,
    /// Chart height
    pub height: u32,
    /// Whether to show legend
    pub show_legend: bool,
    /// Whether to show grid
    pub show_grid: bool,
    /// Chart background color
    pub background_color: String,
    /// Chart type
    pub chart_type: ChartType,
}

/// Chart structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    /// Chart configuration
    pub config: ChartConfig,
    /// Chart datasets
    pub datasets: Vec<ChartDataset>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Chart generator for knowledge data
pub struct ChartGenerator;

impl ChartGenerator {
    /// Create a new chart generator
    pub fn new() -> Self {
        Self
    }

    /// Create a bar chart from data
    pub fn create_bar_chart(
        &self,
        title: &str,
        x_label: &str,
        y_label: &str,
        data: &[(String, f64)],
    ) -> Chart {
        let data_points = data
            .iter()
            .enumerate()
            .map(|(i, (label, value))| DataPoint {
                x: i as f64,
                y: *value,
                label: Some(label.clone()),
                color: None,
                metadata: HashMap::new(),
            })
            .collect();

        let dataset = ChartDataset {
            label: "Data".to_string(),
            data: data_points,
            color: "#4CAF50".to_string(),
            visible: true,
            metadata: HashMap::new(),
        };

        Chart {
            config: ChartConfig {
                title: title.to_string(),
                x_label: x_label.to_string(),
                y_label: y_label.to_string(),
                width: 800,
                height: 600,
                show_legend: true,
                show_grid: true,
                background_color: "#FFFFFF".to_string(),
                chart_type: ChartType::Bar,
            },
            datasets: vec![dataset],
            metadata: HashMap::new(),
        }
    }

    /// Create a line chart from data
    pub fn create_line_chart(
        &self,
        title: &str,
        x_label: &str,
        y_label: &str,
        data: &[(f64, f64)],
    ) -> Chart {
        let data_points = data
            .iter()
            .map(|(x, y)| DataPoint {
                x: *x,
                y: *y,
                label: None,
                color: None,
                metadata: HashMap::new(),
            })
            .collect();

        let dataset = ChartDataset {
            label: "Data".to_string(),
            data: data_points,
            color: "#2196F3".to_string(),
            visible: true,
            metadata: HashMap::new(),
        };

        Chart {
            config: ChartConfig {
                title: title.to_string(),
                x_label: x_label.to_string(),
                y_label: y_label.to_string(),
                width: 800,
                height: 600,
                show_legend: true,
                show_grid: true,
                background_color: "#FFFFFF".to_string(),
                chart_type: ChartType::Line,
            },
            datasets: vec![dataset],
            metadata: HashMap::new(),
        }
    }

    /// Create a pie chart from data
    pub fn create_pie_chart(&self, title: &str, data: &[(String, f64)]) -> Chart {
        let data_points = data
            .iter()
            .enumerate()
            .map(|(i, (label, value))| DataPoint {
                x: i as f64,
                y: *value,
                label: Some(label.clone()),
                color: None,
                metadata: HashMap::new(),
            })
            .collect();

        let dataset = ChartDataset {
            label: "Data".to_string(),
            data: data_points,
            color: "#FF9800".to_string(),
            visible: true,
            metadata: HashMap::new(),
        };

        Chart {
            config: ChartConfig {
                title: title.to_string(),
                x_label: "Categories".to_string(),
                y_label: "Values".to_string(),
                width: 800,
                height: 600,
                show_legend: true,
                show_grid: false,
                background_color: "#FFFFFF".to_string(),
                chart_type: ChartType::Pie,
            },
            datasets: vec![dataset],
            metadata: HashMap::new(),
        }
    }

    /// Create a scatter plot from data
    pub fn create_scatter_plot(
        &self,
        title: &str,
        x_label: &str,
        y_label: &str,
        data: &[(f64, f64)],
    ) -> Chart {
        let data_points = data
            .iter()
            .map(|(x, y)| DataPoint {
                x: *x,
                y: *y,
                label: None,
                color: None,
                metadata: HashMap::new(),
            })
            .collect();

        let dataset = ChartDataset {
            label: "Data".to_string(),
            data: data_points,
            color: "#9C27B0".to_string(),
            visible: true,
            metadata: HashMap::new(),
        };

        Chart {
            config: ChartConfig {
                title: title.to_string(),
                x_label: x_label.to_string(),
                y_label: y_label.to_string(),
                width: 800,
                height: 600,
                show_legend: true,
                show_grid: true,
                background_color: "#FFFFFF".to_string(),
                chart_type: ChartType::Scatter,
            },
            datasets: vec![dataset],
            metadata: HashMap::new(),
        }
    }

    /// Create a histogram from data
    pub fn create_histogram(
        &self,
        title: &str,
        x_label: &str,
        y_label: &str,
        data: &[f64],
        bins: usize,
    ) -> Chart {
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let bin_width = (max_val - min_val) / bins as f64;

        let mut histogram_data = vec![0; bins];
        for &value in data {
            let bin_index = ((value - min_val) / bin_width).floor() as usize;
            let bin_index = bin_index.min(bins - 1);
            histogram_data[bin_index] += 1;
        }

        let data_points = histogram_data
            .iter()
            .enumerate()
            .map(|(i, &count)| DataPoint {
                x: min_val + (i as f64 * bin_width),
                y: count as f64,
                label: Some(format!("{:.2}", min_val + (i as f64 * bin_width))),
                color: None,
                metadata: HashMap::new(),
            })
            .collect();

        let dataset = ChartDataset {
            label: "Frequency".to_string(),
            data: data_points,
            color: "#607D8B".to_string(),
            visible: true,
            metadata: HashMap::new(),
        };

        Chart {
            config: ChartConfig {
                title: title.to_string(),
                x_label: x_label.to_string(),
                y_label: y_label.to_string(),
                width: 800,
                height: 600,
                show_legend: true,
                show_grid: true,
                background_color: "#FFFFFF".to_string(),
                chart_type: ChartType::Histogram,
            },
            datasets: vec![dataset],
            metadata: HashMap::new(),
        }
    }

    /// Export chart as JSON
    pub fn export_as_json(&self, chart: &Chart) -> KnowledgeResult<String> {
        serde_json::to_string_pretty(chart)
            .map_err(|e| crate::types::KnowledgeError::SerdeJsonError(e))
    }

    /// Export chart as SVG
    pub fn export_as_svg(&self, chart: &Chart) -> KnowledgeResult<String> {
        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            chart.config.width, chart.config.height
        );

        // Add background
        svg.push_str(&format!(
            r#"<rect width="{}" height="{}" fill="{}"/>"#,
            chart.config.width, chart.config.height, chart.config.background_color
        ));

        // Add title
        svg.push_str(&format!(
            r#"<text x="{}" y="30" text-anchor="middle" font-size="18" font-weight="bold">{}</text>"#,
            chart.config.width / 2, chart.config.title
        ));

        // Add chart content based on type
        match chart.config.chart_type {
            ChartType::Bar => self.render_bar_chart_svg(chart, &mut svg),
            ChartType::Line => self.render_line_chart_svg(chart, &mut svg),
            ChartType::Pie => self.render_pie_chart_svg(chart, &mut svg),
            ChartType::Scatter => self.render_scatter_plot_svg(chart, &mut svg),
            ChartType::Histogram => self.render_histogram_svg(chart, &mut svg),
            _ => {
                // Default rendering
                svg.push_str(r#"<text x="50" y="100" font-size="14">Chart rendering not implemented for this type</text>"#);
            }
        }

        svg.push_str("</svg>");
        Ok(svg)
    }

    /// Export chart as HTML with Chart.js
    pub fn export_as_html(&self, chart: &Chart) -> KnowledgeResult<String> {
        let json_data = self.export_as_json(chart)?;
        let chart_type = match chart.config.chart_type {
            ChartType::Bar => "bar",
            ChartType::Line => "line",
            ChartType::Pie => "pie",
            ChartType::Scatter => "scatter",
            ChartType::Area => "line",
            ChartType::Histogram => "bar",
            ChartType::Heatmap => "bar",
        };

        let html = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <title>{}</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        .chart-container {{
            width: {}px;
            height: {}px;
            margin: 20px auto;
        }}
    </style>
</head>
<body>
    <div class="chart-container">
        <canvas id="chart"></canvas>
    </div>
    <script>
        const ctx = document.getElementById('chart').getContext('2d');
        const data = {};
        new Chart(ctx, {{
            type: '{}',
            data: data,
            options: {{
                responsive: true,
                maintainAspectRatio: false,
                plugins: {{
                    title: {{
                        display: true,
                        text: '{}'
                    }}
                }}
            }}
        }});
    </script>
</body>
</html>
"#,
            chart.config.title,
            chart.config.width,
            chart.config.height,
            json_data,
            chart_type,
            chart.config.title
        );
        Ok(html)
    }

    /// Render bar chart as SVG
    fn render_bar_chart_svg(&self, chart: &Chart, svg: &mut String) {
        if let Some(dataset) = chart.datasets.first() {
            let chart_width = chart.config.width as f64 - 100.0;
            let chart_height = chart.config.height as f64 - 100.0;
            let bar_width = chart_width / dataset.data.len() as f64 * 0.8;
            let max_value = dataset.data.iter().map(|p| p.y).fold(0.0, f64::max);

            for (i, point) in dataset.data.iter().enumerate() {
                let x = 50.0 + (i as f64 * chart_width / dataset.data.len() as f64);
                let bar_height = (point.y / max_value) * chart_height;
                let y = chart.config.height as f64 - 50.0 - bar_height;

                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="black" stroke-width="1"/>"#,
                    x, y, bar_width, bar_height, dataset.color
                ));

                if let Some(label) = &point.label {
                    svg.push_str(&format!(
                        r#"<text x="{}" y="{}" text-anchor="middle" font-size="10">{}</text>"#,
                        x + bar_width / 2.0,
                        chart.config.height as f64 - 30.0,
                        label
                    ));
                }
            }
        }
    }

    /// Render line chart as SVG
    fn render_line_chart_svg(&self, chart: &Chart, svg: &mut String) {
        if let Some(dataset) = chart.datasets.first() {
            let chart_width = chart.config.width as f64 - 100.0;
            let chart_height = chart.config.height as f64 - 100.0;
            let max_x = dataset.data.iter().map(|p| p.x).fold(0.0, f64::max);
            let max_y = dataset.data.iter().map(|p| p.y).fold(0.0, f64::max);

            let mut path_data = String::new();
            for (i, point) in dataset.data.iter().enumerate() {
                let x = 50.0 + (point.x / max_x) * chart_width;
                let y = chart.config.height as f64 - 50.0 - (point.y / max_y) * chart_height;

                if i == 0 {
                    path_data.push_str(&format!("M {} {}", x, y));
                } else {
                    path_data.push_str(&format!(" L {} {}", x, y));
                }
            }

            svg.push_str(&format!(
                r#"<path d="{}" stroke="{}" stroke-width="2" fill="none"/>"#,
                path_data, dataset.color
            ));
        }
    }

    /// Render pie chart as SVG
    fn render_pie_chart_svg(&self, chart: &Chart, svg: &mut String) {
        if let Some(dataset) = chart.datasets.first() {
            let center_x = chart.config.width as f64 / 2.0;
            let center_y = chart.config.height as f64 / 2.0;
            let radius = 150.0;
            let total = dataset.data.iter().map(|p| p.y).sum::<f64>();

            let mut current_angle = 0.0;
            for point in &dataset.data {
                let slice_angle = (point.y / total) * 2.0 * std::f64::consts::PI;
                let end_angle = current_angle + slice_angle;

                let x1 = center_x + radius * current_angle.cos();
                let y1 = center_y + radius * current_angle.sin();
                let x2 = center_x + radius * end_angle.cos();
                let y2 = center_y + radius * end_angle.sin();

                let large_arc_flag = if slice_angle > std::f64::consts::PI {
                    1
                } else {
                    0
                };

                let path_data = format!(
                    "M {} {} A {} {} 0 {} 1 {} {} L {} {} Z",
                    center_x, center_y, radius, radius, large_arc_flag, x2, y2, center_x, center_y
                );

                svg.push_str(&format!(
                    r#"<path d="{}" fill="{}" stroke="black" stroke-width="1"/>"#,
                    path_data, dataset.color
                ));

                current_angle = end_angle;
            }
        }
    }

    /// Render scatter plot as SVG
    fn render_scatter_plot_svg(&self, chart: &Chart, svg: &mut String) {
        if let Some(dataset) = chart.datasets.first() {
            let chart_width = chart.config.width as f64 - 100.0;
            let chart_height = chart.config.height as f64 - 100.0;
            let max_x = dataset.data.iter().map(|p| p.x).fold(0.0, f64::max);
            let max_y = dataset.data.iter().map(|p| p.y).fold(0.0, f64::max);

            for point in &dataset.data {
                let x = 50.0 + (point.x / max_x) * chart_width;
                let y = chart.config.height as f64 - 50.0 - (point.y / max_y) * chart_height;

                svg.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="3" fill="{}" stroke="black" stroke-width="1"/>"#,
                    x, y, dataset.color
                ));
            }
        }
    }

    /// Render histogram as SVG
    fn render_histogram_svg(&self, chart: &Chart, svg: &mut String) {
        self.render_bar_chart_svg(chart, svg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_chart_creation() {
        let generator = ChartGenerator::new();
        let data = vec![
            ("A".to_string(), 10.0),
            ("B".to_string(), 20.0),
            ("C".to_string(), 15.0),
        ];

        let chart = generator.create_bar_chart("Test Chart", "Categories", "Values", &data);
        assert_eq!(chart.config.title, "Test Chart");
        assert_eq!(chart.config.chart_type, ChartType::Bar);
        assert_eq!(chart.datasets.len(), 1);
        assert_eq!(chart.datasets[0].data.len(), 3);
    }

    #[test]
    fn test_line_chart_creation() {
        let generator = ChartGenerator::new();
        let data = vec![(1.0, 10.0), (2.0, 20.0), (3.0, 15.0)];

        let chart = generator.create_line_chart("Test Chart", "X", "Y", &data);
        assert_eq!(chart.config.title, "Test Chart");
        assert_eq!(chart.config.chart_type, ChartType::Line);
        assert_eq!(chart.datasets.len(), 1);
        assert_eq!(chart.datasets[0].data.len(), 3);
    }

    #[test]
    fn test_pie_chart_creation() {
        let generator = ChartGenerator::new();
        let data = vec![
            ("A".to_string(), 30.0),
            ("B".to_string(), 40.0),
            ("C".to_string(), 30.0),
        ];

        let chart = generator.create_pie_chart("Test Chart", &data);
        assert_eq!(chart.config.title, "Test Chart");
        assert_eq!(chart.config.chart_type, ChartType::Pie);
        assert_eq!(chart.datasets.len(), 1);
        assert_eq!(chart.datasets[0].data.len(), 3);
    }

    #[test]
    fn test_histogram_creation() {
        let generator = ChartGenerator::new();
        let data = vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 5.0];

        let chart = generator.create_histogram("Test Histogram", "Values", "Frequency", &data, 5);
        assert_eq!(chart.config.title, "Test Histogram");
        assert_eq!(chart.config.chart_type, ChartType::Histogram);
        assert_eq!(chart.datasets.len(), 1);
    }

    #[test]
    fn test_json_export() {
        let generator = ChartGenerator::new();
        let data = vec![("A".to_string(), 10.0), ("B".to_string(), 20.0)];
        let chart = generator.create_bar_chart("Test", "X", "Y", &data);

        let json = generator.export_as_json(&chart).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("Bar"));
    }
}
