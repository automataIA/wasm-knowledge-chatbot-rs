use crate::graphrag_config::GraphRAGConfig;
use crate::models::app::AppResult;
use crate::models::graph_store::GraphStore;
use crate::models::graphrag::{DocumentIndex, ProcessingStatus, RAGQuery, RAGResult};
use crate::utils::storage::StorageUtils;

/// Pipeline entrypoints for GraphRAG. Honors configuration when indexing/querying.
pub struct GraphRAGPipeline {
    config: GraphRAGConfig,
}

impl GraphRAGPipeline {
    pub fn new() -> Self {
        // Load GraphRAGConfig from localStorage; prefer v1 key with legacy fallback
        let config = if let Ok(Some(c)) =
            StorageUtils::retrieve_local::<GraphRAGConfig>("graphrag_config_v1")
        {
            c
        } else {
            match StorageUtils::retrieve_local::<GraphRAGConfig>("graphrag_config") {
                Ok(Some(c)) => c,
                _ => GraphRAGConfig::default(),
            }
        };
        Self { config }
    }

    /// Storage keys for persisted document index (versioned)
    const INDEX_KEY_V1: &'static str = "graphrag_document_index_v1";
    const INDEX_KEY_LEGACY: &'static str = "graphrag_document_index";

    /// Load the current document index from localStorage.
    fn load_index(&self) -> AppResult<Vec<DocumentIndex>> {
        // Prefer v1 key; if missing, fallback to legacy key (migration read)
        if let Ok(Some(v)) = StorageUtils::retrieve_local::<Vec<DocumentIndex>>(Self::INDEX_KEY_V1)
        {
            return Ok(v);
        }
        let legacy = StorageUtils::retrieve_local::<Vec<DocumentIndex>>(Self::INDEX_KEY_LEGACY)?;
        Ok(legacy.unwrap_or_default())
    }

    /// Save the document index to localStorage.
    fn save_index(&self, docs: &[DocumentIndex]) -> AppResult<()> {
        // Write to versioned key
        StorageUtils::store_local(Self::INDEX_KEY_V1, &docs)
    }

    /// Index documents into the knowledge graph.
    /// Current behavior: upsert provided DocumentIndex entries by id and persist.
    pub fn index_documents(&self, docs: &[DocumentIndex]) -> AppResult<()> {
        // Load existing
        let mut existing = self.load_index()?;

        // Honor batch_size: process in chunks and annotate processing_status/indexed_at
        let now = js_sys::Date::now();
        let batch = self.config.batch_size.max(1);
        for chunk in docs.chunks(batch) {
            for d in chunk.iter().cloned() {
                // Mark completed with updated timestamp
                let mut updated = d;
                updated.indexed_at = now;
                updated.processing_status = ProcessingStatus::Completed;
                if let Some(slot) = existing.iter_mut().find(|x| x.id == updated.id) {
                    *slot = updated;
                } else {
                    existing.push(updated);
                }
            }
        }

        // Persist
        self.save_index(&existing)
    }

    /// Delete a single document by id from the persisted index and cascade-remove
    /// associated nodes/edges from the GraphStore.
    pub fn delete_document_by_id(&self, id: &str) -> AppResult<()> {
        // Load existing index
        let mut existing = self.load_index()?;
        // Filter out the document
        let before = existing.len();
        existing.retain(|d| d.id != id);
        // Persist index only if changed
        if existing.len() != before {
            self.save_index(&existing)?;
        }
        // Remove from graph store (best-effort)
        if let Ok(mut store) = GraphStore::load() {
            store.remove_document_cascade(id);
            let _ = store.save();
        }
        Ok(())
    }

    /// Delete multiple documents by ids. Returns Ok even if some ids were not present.
    pub fn delete_documents_by_ids(&self, ids: &[String]) -> AppResult<()> {
        if ids.is_empty() {
            return Ok(());
        }
        let mut existing = self.load_index()?;
        let idset: std::collections::HashSet<&String> = ids.iter().collect();
        let before = existing.len();
        existing.retain(|d| !idset.contains(&d.id));
        if existing.len() != before {
            self.save_index(&existing)?;
        }
        if let Ok(mut store) = GraphStore::load() {
            for id in ids {
                store.remove_document_cascade(id);
            }
            let _ = store.save();
        }
        Ok(())
    }

    /// Run a GraphRAG query against the current index. Stub: returns empty result.
    pub async fn query(&self, q: &RAGQuery) -> RAGResult {
        RAGResult {
            id: q.id.clone(),
            query_id: q.id.clone(),
            nodes: vec![],
            edges: vec![],
            scores: vec![],
            metadata: crate::models::graphrag::ResultMetadata {
                processing_time_ms: 0,
                total_nodes_searched: 0,
                reranked: false,
                hyde_enhanced: false,
                community_filtered: false,
                algorithms_used: vec![],
                summary: None,
            },
        }
    }
}

impl Default for GraphRAGPipeline {
    fn default() -> Self {
        Self::new()
    }
}
