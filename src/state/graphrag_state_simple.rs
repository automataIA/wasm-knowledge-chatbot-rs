use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use crate::models::{app::AppError, graphrag::{RAGQuery, RAGResult, SearchStrategy}};
use crate::features::graphrag::{GraphRAGPipeline, Retriever};
use crate::features::graphrag::extraction::extract_entities_relations;
use std::collections::HashSet;
use crate::state::knowledge_storage_context::KnowledgeStorageContext;

#[derive(Clone)]
pub struct GraphRAGStateContext {
    indexing: RwSignal<bool>,
    searching: RwSignal<bool>,
    last_error: RwSignal<Option<AppError>>,
    last_result: RwSignal<Option<RAGResult>>,
    index_progress: RwSignal<Option<f32>>, // 0.0..=1.0 when indexing
}

impl Default for GraphRAGStateContext {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphRAGStateContext {
    pub fn new() -> Self {
        Self {
            indexing: RwSignal::new(false),
            searching: RwSignal::new(false),
            last_error: RwSignal::new(None),
            last_result: RwSignal::new(None),
            index_progress: RwSignal::new(None),
        }
    }

    pub fn is_indexing(&self) -> ReadSignal<bool> { self.indexing.read_only() }
    pub fn is_searching(&self) -> ReadSignal<bool> { self.searching.read_only() }
    pub fn last_error(&self) -> ReadSignal<Option<AppError>> { self.last_error.read_only() }
    pub fn last_result(&self) -> ReadSignal<Option<RAGResult>> { self.last_result.read_only() }
    pub fn index_progress(&self) -> ReadSignal<Option<f32>> { self.index_progress.read_only() }

    // Convenience getters for tests and non-reactive checks
    pub fn indexing_now(&self) -> bool { self.indexing.get() }
    pub fn searching_now(&self) -> bool { self.searching.get() }
    pub fn last_error_now(&self) -> Option<AppError> { self.last_error.get() }
    pub fn last_result_now(&self) -> Option<RAGResult> { self.last_result.get() }
    pub fn index_progress_now(&self) -> Option<f32> { self.index_progress.get() }

    pub fn set_error(&self, err: Option<AppError>) { self.last_error.set(err); }

    pub fn run_query(&self, q: RAGQuery, strategy: SearchStrategy) {
        let this = self.clone();
        // clear previous error and mark as busy
        this.last_error.set(None);
        this.searching.set(true);
        spawn_local(async move {
            let retriever = Retriever::new();
            let res = retriever.search(&q, strategy).await;
            this.last_result.set(Some(res));
            this.searching.set(false);
        });
    }

    pub fn reindex(&self) {
        // Placeholder for kicking indexing pipeline.
        let this = self.clone();
        self.indexing.set(true);
        this.index_progress.set(Some(0.0));
        spawn_local(async move {
            let pipeline = GraphRAGPipeline::new();
            // Load real documents for indexing from shared storage context via Leptos context
            let kctx: KnowledgeStorageContext = use_context().unwrap_or_default();
            let docs = kctx.get_documents_for_indexing();
            // Simulate progress in a few steps
            async fn sleep_ms(ms: i32) {
                let p = Promise::new(&mut |resolve, _reject| {
                    let _ = window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
                });
                let _ = JsFuture::from(p).await;
            }

            sleep_ms(150).await;
            this.index_progress.set(Some(0.3));
            sleep_ms(200).await;
            this.index_progress.set(Some(0.7));

            // Index the collected documents
            let _ = pipeline.index_documents(&docs);

            // Extract simple entities/relations and persist to GraphStore (basic migration if empty)
            let (nodes, edges) = extract_entities_relations(&docs);
            let _ = kctx.update_graph_store(|store| {
                let mut existing_node_ids: HashSet<String> = store.nodes.iter().map(|n| n.id.clone()).collect();
                let mut existing_edge_ids: HashSet<String> = store.edges.iter().map(|e| e.id.clone()).collect();
                for n in &nodes {
                    if existing_node_ids.insert(n.id.clone()) { store.nodes.push(n.clone()); }
                }
                for e in &edges {
                    if existing_edge_ids.insert(e.id.clone()) { store.edges.push(e.clone()); }
                }
            });

            this.index_progress.set(Some(1.0));
            sleep_ms(120).await;
            this.index_progress.set(None);
            this.indexing.set(false);
        });
    }
}

#[component]
pub fn GraphRAGStateProvider(children: Children) -> impl IntoView {
    let ctx = GraphRAGStateContext::new();
    provide_context(ctx);
    children()
}

pub fn use_graphrag_state() -> GraphRAGStateContext {
    expect_context::<GraphRAGStateContext>()
}
