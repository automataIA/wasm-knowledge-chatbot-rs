use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub content: String,
    pub node_type: NodeType,
    pub metadata: NodeMetadata,
    pub embeddings: Option<Vec<f32>>,
    pub connections: Vec<String>, // Node IDs
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    Document,
    Entity,
    Concept,
    Relationship,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub created_at: f64,
    pub updated_at: f64,
    pub source: Option<String>,
    pub confidence: f32, // 0.0 to 1.0
    pub tags: Vec<String>,
    pub properties: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub edge_type: EdgeType,
    pub weight: f32,
    pub metadata: EdgeMetadata,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EdgeType {
    References,
    Contains,
    RelatedTo,
    Causes,
    PartOf,
    Custom(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EdgeMetadata {
    pub created_at: f64,
    pub confidence: f32,
    pub properties: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RAGQuery {
    pub id: String,
    pub text: String,
    pub query_type: QueryType,
    pub filters: QueryFilters,
    pub config: QueryConfig,
    pub timestamp: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryType {
    Semantic,
    Keyword,
    Hybrid,
    Graph,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryFilters {
    pub node_types: Vec<NodeType>,
    pub tags: Vec<String>,
    pub date_range: Option<(f64, f64)>,
    pub confidence_threshold: Option<f32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueryConfig {
    pub max_results: usize,
    pub similarity_threshold: f32,
    pub use_reranking: bool,
    pub use_hyde: bool,
    pub use_community_detection: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RAGResult {
    pub id: String,
    pub query_id: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub scores: Vec<f32>,
    pub metadata: ResultMetadata,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResultMetadata {
    pub processing_time_ms: u32,
    pub total_nodes_searched: usize,
    pub reranked: bool,
    pub hyde_enhanced: bool,
    pub community_filtered: bool,
    pub algorithms_used: Vec<String>,
    pub summary: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentIndex {
    pub id: String,
    pub title: String,
    pub content: String,
    pub file_type: String,
    pub size_bytes: u64,
    pub created_at: f64,
    pub indexed_at: f64,
    pub node_count: usize,
    pub embedding_model: Option<String>,
    pub processing_status: ProcessingStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProcessingStatus {
    Pending,
    Processing { progress: f32 },
    Completed,
    Failed { error: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum SearchStrategy {
    Automatic,
    Local,
    Global,
    Combined,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PerformanceMode {
    MaxQuality,
    Balanced,
    MaxSpeed,
}

impl Default for QueryFilters {
    fn default() -> Self {
        Self {
            node_types: Vec::new(),
            tags: Vec::new(),
            date_range: None,
            confidence_threshold: Some(0.3),
        }
    }
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self {
            max_results: 10,
            similarity_threshold: 0.7,
            use_reranking: false,
            use_hyde: true,
            use_community_detection: true,
        }
    }
}

impl RAGQuery {
    pub fn new(text: String) -> Self {
        Self {
            id: format!("{}", js_sys::Date::now()),
            text,
            query_type: QueryType::Hybrid,
            filters: QueryFilters::default(),
            config: QueryConfig::default(),
            timestamp: js_sys::Date::now(),
        }
    }
}

impl GraphNode {
    pub fn new(content: String, node_type: NodeType) -> Self {
        let timestamp = js_sys::Date::now();
        Self {
            id: format!("{}", timestamp),
            content,
            node_type,
            metadata: NodeMetadata {
                created_at: timestamp,
                updated_at: timestamp,
                source: None,
                confidence: 1.0,
                tags: Vec::new(),
                properties: HashMap::new(),
            },
            embeddings: None,
            connections: Vec::new(),
        }
    }
}

impl ProcessingStatus {
    pub fn is_completed(&self) -> bool {
        matches!(self, ProcessingStatus::Completed)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, ProcessingStatus::Failed { .. })
    }

    pub fn progress(&self) -> Option<f32> {
        match self {
            ProcessingStatus::Processing { progress } => Some(*progress),
            _ => None,
        }
    }
}
