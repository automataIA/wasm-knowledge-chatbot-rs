use wasm_bindgen_test::*;
use web_sys::{window, Storage};

use wasm_knowledge_chatbot_rs::features::graphrag::retrieval::Retriever;
use wasm_knowledge_chatbot_rs::graphrag_config::{create_graphrag_signals, with_graphrag_manager};
use wasm_knowledge_chatbot_rs::models::graphrag::{
    DocumentIndex, ProcessingStatus, RAGQuery, SearchStrategy,
};

wasm_bindgen_test_configure!(run_in_browser);

fn storage() -> Storage {
    window().unwrap().local_storage().unwrap().unwrap()
}

fn seed_docs() {
    let docs = vec![
        DocumentIndex {
            id: "doc1".to_string(),
            title: "Rust GraphRAG Intro".to_string(),
            content: "Graph-based retrieval augments RAG with nodes and edges.".to_string(),
            file_type: "md".to_string(),
            size_bytes: 1000,
            created_at: 0.0,
            indexed_at: 0.0,
            node_count: 1,
            embedding_model: None,
            processing_status: ProcessingStatus::Completed,
        },
        DocumentIndex {
            id: "doc2".to_string(),
            title: "Community detection and PageRank".to_string(),
            content: "Louvain communities and pagerank centrality improve retrieval.".to_string(),
            file_type: "md".to_string(),
            size_bytes: 1200,
            created_at: 0.0,
            indexed_at: 0.0,
            node_count: 1,
            embedding_model: None,
            processing_status: ProcessingStatus::Completed,
        },
        DocumentIndex {
            id: "doc3".to_string(),
            title: "HyDE synthetic query expansion".to_string(),
            content: "HyDE expands queries by generating hypothetical variants.".to_string(),
            file_type: "md".to_string(),
            size_bytes: 900,
            created_at: 0.0,
            indexed_at: 0.0,
            node_count: 1,
            embedding_model: None,
            processing_status: ProcessingStatus::Completed,
        },
    ];
    let json = serde_json::to_string(&docs).unwrap();
    let _ = storage().set_item("graphrag_document_index_v1", &json);
}

#[wasm_bindgen_test(async)]
async fn metrics_update_and_toggles_flow() {
    // Initialize manager and expose globally
    let (_cfg_sig, _met_sig, manager) = create_graphrag_signals();

    // Ensure defaults persisted
    manager.reset_to_defaults();

    // Seed docs into localStorage for retrieval to read
    seed_docs();

    // Configure toggles ON to exercise all branches
    manager.update_config(|c| {
        c.hyde_enabled = true;
        c.community_detection_enabled = true;
        c.pagerank_enabled = true;
        c.reranking_enabled = true;
        c.synthesis_enabled = true;
        c.max_query_time_ms = 10_000;
    });

    // Build query
    let mut q = RAGQuery::new("pagerank community hyde".to_string());
    q.config.max_results = 3;
    q.config.use_hyde = true;
    q.config.use_community_detection = true;
    q.config.use_reranking = true;

    // Run retrieval
    let r = Retriever::new().search(&q, SearchStrategy::Automatic).await;

    // Assertions on result metadata affected by toggles
    assert!(r.metadata.processing_time_ms > 0);
    // algorithms present when toggles ON
    let alg = &r.metadata.algorithms_used;
    assert!(alg.iter().any(|a| a == "tfidf"));
    assert!(alg.iter().any(|a| a == "hyde"));
    assert!(alg.iter().any(|a| a == "community_boost"));
    assert!(alg.iter().any(|a| a == "pagerank_weighting"));
    assert!(alg.iter().any(|a| a == "advanced_rerank"));
    assert!(alg.iter().any(|a| a == "synthesis"));
    // flags
    assert!(r.metadata.hyde_enhanced);
    assert!(r.metadata.community_filtered);
    assert!(r.metadata.reranked);
    assert!(r.metadata.summary.is_some());

    // Metrics should be updated via global manager
    with_graphrag_manager(|m| {
        let perf = m.get_performance_metrics();
        let met = m.get_metrics();
        assert!(perf.total_time_ms >= r.metadata.processing_time_ms);
        // at least one of the stage timers should be set when features are enabled
        assert!(
            perf.hyde_time_ms > 0
                || perf.community_detection_time_ms > 0
                || perf.pagerank_time_ms > 0
                || perf.reranking_time_ms > 0
                || perf.synthesis_time_ms > 0
        );
        assert!(met.last_query_time_ms >= r.metadata.processing_time_ms);
        assert!(met.queries_processed >= 1);
        assert!(met.active_features.iter().any(|f| f == "HyDE"));
    });

    // Now turn OFF toggles and observe metadata differences
    manager.update_config(|c| {
        c.hyde_enabled = false;
        c.community_detection_enabled = false;
        c.pagerank_enabled = false;
        c.reranking_enabled = false;
        c.synthesis_enabled = false;
    });

    let mut q2 = RAGQuery::new("pagerank community hyde".to_string());
    q2.config.max_results = 3;
    q2.config.use_hyde = false;
    q2.config.use_community_detection = false;
    q2.config.use_reranking = false;

    let r2 = Retriever::new()
        .search(&q2, SearchStrategy::Automatic)
        .await;

    assert!(!r2.metadata.hyde_enhanced);
    assert!(!r2.metadata.community_filtered);
    assert!(!r2.metadata.reranked);
    assert!(r2.metadata.algorithms_used.iter().any(|a| a == "tfidf"));
    // With synthesis disabled no summary expected
    assert!(r2.metadata.summary.is_none());
}

#[wasm_bindgen_test(async)]
async fn empty_index_graceful_handling() {
    // Initialize manager
    let (_c, _m, manager) = create_graphrag_signals();
    manager.reset_to_defaults();

    // Clear indices
    let _ = storage().remove_item("graphrag_document_index_v1");
    let _ = storage().remove_item("graphrag_document_index");

    let mut q = RAGQuery::new("anything".to_string());
    q.config.max_results = 3;
    let r = Retriever::new().search(&q, SearchStrategy::Automatic).await;

    assert_eq!(r.nodes.len(), 0);
    assert_eq!(r.edges.len(), 0);
    assert_eq!(r.scores.len(), 0);
    assert!(r.metadata.processing_time_ms > 0);
    assert!(r.metadata.algorithms_used.iter().any(|a| a == "tfidf"));
}

#[wasm_bindgen_test(async)]
async fn context_bridging_metrics_persistence() {
    let (_cfg_sig, _met_sig, manager) = create_graphrag_signals();
    manager.reset_to_defaults();
    seed_docs();

    // Snapshot metrics before
    let before = manager.get_metrics();

    let mut q = RAGQuery::new("pagerank".to_string());
    q.config.max_results = 2;
    let _ = Retriever::new().search(&q, SearchStrategy::Automatic).await;

    // After one search
    with_graphrag_manager(|m| {
        let met = m.get_metrics();
        assert!(met.queries_processed > before.queries_processed);
        assert!(met.last_query_time_ms > 0);
        // Active features list updated from defaults
        assert!(!met.active_features.is_empty());
    });
}
