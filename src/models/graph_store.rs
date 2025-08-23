use crate::models::app::AppError;
use crate::utils::storage::StorageUtils;
use serde::{Deserialize, Serialize};

pub const GRAPH_STORE_KEY_V1: &str = "graphrag_graph_store_v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphNode {
    pub id: String,
    pub label: Option<String>,
    pub node_type: String,
    pub source_document_id: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphEdge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub relation: String,
    pub weight: f32,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GraphStore {
    pub version: u32,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl GraphStore {
    pub fn new() -> Self {
        Self {
            version: 1,
            nodes: vec![],
            edges: vec![],
        }
    }
    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.push(node);
    }
    pub fn add_edge(&mut self, edge: GraphEdge) {
        self.edges.push(edge);
    }
    pub fn save(&self) -> Result<(), AppError> {
        StorageUtils::store_local(GRAPH_STORE_KEY_V1, self)
    }
    pub fn load() -> Result<Self, AppError> {
        Ok(StorageUtils::retrieve_local(GRAPH_STORE_KEY_V1)?.unwrap_or_default())
    }

    /// Remove all nodes and edges associated with a given document id.
    /// This will:
    /// - Remove nodes whose `id` equals the document id
    /// - Remove nodes whose `source_document_id` equals the document id
    /// - Remove edges that touch the removed nodes
    /// - Remove edges whose `from` or `to` equals the document id
    pub fn remove_document_cascade(&mut self, document_id: &str) {
        // Collect node ids to remove: direct match or by source_document_id
        let mut remove_node_ids: Vec<String> = Vec::new();
        for n in &self.nodes {
            if n.id == document_id || n.source_document_id.as_deref() == Some(document_id) {
                remove_node_ids.push(n.id.clone());
            }
        }

        if remove_node_ids.is_empty() {
            // Still ensure we drop edges pointing directly to the document id
            self.edges
                .retain(|e| e.from != document_id && e.to != document_id);
            return;
        }

        // Remove nodes
        let remove_set: std::collections::HashSet<String> =
            remove_node_ids.iter().cloned().collect();
        self.nodes.retain(|n| !remove_set.contains(&n.id));

        // Remove edges touching removed nodes or the document id directly
        self.edges.retain(|e| {
            let touches_removed = remove_set.contains(&e.from) || remove_set.contains(&e.to);
            let touches_doc = e.from == document_id || e.to == document_id;
            !(touches_removed || touches_doc)
        });
    }
}
