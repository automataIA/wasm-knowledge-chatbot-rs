use crate::features::graphrag::traversal::{bfs, dfs, TraversalFilters, TraversalResult};
use crate::models::app::AppError;
use crate::models::graph_store::GraphStore;
use crate::models::graphrag::{DocumentIndex, ProcessingStatus};
use crate::utils::storage::StorageUtils;

/// Minimal shared storage context that exposes documents for GraphRAG indexing.
/// It reads a plain text buffer saved by the Document Manager from localStorage
/// and converts it into `DocumentIndex` entries.
#[derive(Clone, Default)]
pub struct KnowledgeStorageContext;

impl KnowledgeStorageContext {
    pub fn new() -> Self {
        Self
    }

    /// Storage key where Document Manager persists the aggregated uploaded content.
    const BUFFER_KEY: &'static str = "knowledge_upload_buffer_v1";

    /// Load the raw buffer from localStorage.
    fn load_buffer(&self) -> Option<String> {
        match StorageUtils::retrieve_local::<String>(Self::BUFFER_KEY) {
            Ok(Some(s)) => Some(s),
            _ => None,
        }
    }

    /// Parse the raw buffer into `DocumentIndex` entries.
    /// The buffer format is a simple concatenation of segments:
    ///   "# File: <name>\n\n<content>\n\n---\n\n# File: ..."
    pub fn get_documents_for_indexing(&self) -> Vec<DocumentIndex> {
        let mut out = Vec::new();
        let now = js_sys::Date::now();
        let Some(buf) = self.load_buffer() else {
            return out;
        };

        // Split by separator used in DocumentManagerSimple
        let segments = buf.split("\n\n---\n\n");
        for seg in segments {
            let seg = seg.trim();
            if seg.is_empty() {
                continue;
            }
            if let Some(rest) = seg.strip_prefix("# File:") {
                // Extract name and content
                let mut lines = rest.lines();
                // The first line after '# File:' contains the name (possibly prefixed by a space)
                let name_line = lines.next().unwrap_or("").trim();
                let title = name_line.to_string();
                // Remaining lines form the content (skip possible empty line)
                let content: String = lines.collect::<Vec<_>>().join("\n");
                let content = content.trim_start_matches('\n').to_string();

                if title.is_empty() && content.is_empty() {
                    continue;
                }

                let file_type = if title.ends_with(".md") || title.ends_with(".markdown") {
                    "markdown"
                } else if title.ends_with(".txt") {
                    "text"
                } else {
                    "unknown"
                };
                let size_bytes = content.len() as u64;

                out.push(DocumentIndex {
                    id: format!("{}:{}", now, title),
                    title,
                    content,
                    file_type: file_type.to_string(),
                    size_bytes,
                    created_at: now,
                    indexed_at: now,
                    node_count: 0,
                    embedding_model: None,
                    processing_status: ProcessingStatus::Pending,
                });
            } else {
                // Fallback: treat whole segment as a single unnamed document
                let content = seg.to_string();
                if content.is_empty() {
                    continue;
                }
                out.push(DocumentIndex {
                    id: format!("{}:untitled", now),
                    title: "Untitled".to_string(),
                    content,
                    file_type: "unknown".to_string(),
                    size_bytes: seg.len() as u64,
                    created_at: now,
                    indexed_at: now,
                    node_count: 0,
                    embedding_model: None,
                    processing_status: ProcessingStatus::Pending,
                });
            }
        }
        out
    }

    /// Load the current GraphStore from versioned localStorage or return an empty default store.
    pub fn load_graph_store(&self) -> Result<GraphStore, AppError> {
        GraphStore::load()
    }

    /// Persist a GraphStore to versioned localStorage.
    pub fn save_graph_store(&self, store: &GraphStore) -> Result<(), AppError> {
        store.save()
    }

    /// Convenience: load, mutate via closure, then save.
    pub fn update_graph_store<F>(&self, mutator: F) -> Result<GraphStore, AppError>
    where
        F: FnOnce(&mut GraphStore),
    {
        let mut store = self.load_graph_store()?;
        mutator(&mut store);
        self.save_graph_store(&store)?;
        Ok(store)
    }

    /// Run a BFS traversal over the persisted GraphStore starting at `start_id`.
    /// You can restrict by `allowed_relations` and set traversal limits.
    pub fn traverse_bfs(
        &self,
        start_id: &str,
        allowed_relations: Option<&[String]>,
        max_depth: Option<usize>,
        max_nodes: Option<usize>,
        max_edges: Option<usize>,
    ) -> Result<TraversalResult, AppError> {
        let store = self.load_graph_store()?;
        // Build filters. Clone allowed_relations into an owned Vec so we can take a slice.
        let owned_allowed: Option<Vec<String>> = allowed_relations.map(|a| a.to_vec());
        let filters = TraversalFilters {
            allowed_relations: owned_allowed.as_deref(),
            max_depth,
            max_nodes,
            max_edges,
        };
        Ok(bfs(&store, start_id, &filters))
    }

    /// Run a DFS traversal over the persisted GraphStore starting at `start_id`.
    /// You can restrict by `allowed_relations` and set traversal limits.
    pub fn traverse_dfs(
        &self,
        start_id: &str,
        allowed_relations: Option<&[String]>,
        max_depth: Option<usize>,
        max_nodes: Option<usize>,
        max_edges: Option<usize>,
    ) -> Result<TraversalResult, AppError> {
        let store = self.load_graph_store()?;
        let owned_allowed: Option<Vec<String>> = allowed_relations.map(|a| a.to_vec());
        let filters = TraversalFilters {
            allowed_relations: owned_allowed.as_deref(),
            max_depth,
            max_nodes,
            max_edges,
        };
        Ok(dfs(&store, start_id, &filters))
    }
}
