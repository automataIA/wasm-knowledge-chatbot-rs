use crate::models::graphrag::{GraphNode, GraphEdge};

/// Placeholder for community detection and pagerank functions.
pub struct GraphAnalytics;

impl GraphAnalytics {
    pub fn new() -> Self { Self }

    /// Compute PageRank scores. Stub: returns uniform scores.
    pub fn pagerank(&self, nodes: &[GraphNode], _edges: &[GraphEdge]) -> Vec<f32> {
        if nodes.is_empty() { return vec![]; }
        vec![1.0 / nodes.len() as f32; nodes.len()]
    }
}

impl Default for GraphAnalytics {
    fn default() -> Self { Self::new() }
}
