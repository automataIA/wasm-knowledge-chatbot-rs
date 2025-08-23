use wasm_bindgen_test::*;
use wasm_knowledge_chatbot_rs::features::graphrag::{GraphRAGPipeline, Retriever};
use wasm_knowledge_chatbot_rs::models::graphrag::{DocumentIndex, RAGQuery, SearchStrategy, ProcessingStatus};

wasm_bindgen_test_configure!(run_in_browser);

fn now() -> f64 { js_sys::Date::now() }

fn seed_docs() -> Vec<DocumentIndex> {
    let t = now();
    vec![
        DocumentIndex { id: "d1".into(), title: "Intro to GraphRAG".into(), content: "Graph based RAG retrieval with tfidf and cooccurrence".into(), file_type: "text".into(), size_bytes: 100, created_at: t, indexed_at: t, node_count: 1, embedding_model: None, processing_status: ProcessingStatus::Completed },
        DocumentIndex { id: "d2".into(), title: "WebLLM hooks".into(), content: "Reranking hooks and ui toggles".into(), file_type: "text".into(), size_bytes: 120, created_at: t, indexed_at: t, node_count: 1, embedding_model: None, processing_status: ProcessingStatus::Completed },
        DocumentIndex { id: "d3".into(), title: "Co-occurrence edges".into(), content: "Edges computed by jaccard similarity threshold".into(), file_type: "text".into(), size_bytes: 130, created_at: t, indexed_at: t, node_count: 1, embedding_model: None, processing_status: ProcessingStatus::Completed },
    ]
}

#[wasm_bindgen_test(async)]
async fn end_to_end_search_and_rerank_metadata() {
    // Seed small index
    let pipeline = GraphRAGPipeline::new();
    let docs = seed_docs();
    pipeline.index_documents(&docs).expect("indexing should succeed");

    // Run a combined search without reranking
    let mut q1 = RAGQuery::new("GraphRAG hooks".into());
    q1.config.use_reranking = false;
    let r1 = Retriever::new().search(&q1, SearchStrategy::Combined).await;
    assert!(!r1.nodes.is_empty(), "should return at least one node");
    assert!(r1.metadata.algorithms_used.contains(&"tfidf".to_string()));
    assert!(!r1.metadata.reranked, "reranked should be false when flag is off");

    // Run with reranking
    let mut q2 = RAGQuery::new("GraphRAG hooks".into());
    q2.config.use_reranking = true;
    let r2 = Retriever::new().search(&q2, SearchStrategy::Combined).await;
    assert!(!r2.nodes.is_empty(), "should return at least one node");
    assert!(r2.metadata.algorithms_used.iter().any(|a| a == "rerank_hook"));
    assert!(r2.metadata.reranked, "reranked should be true when flag is on");

    // Edges and weights sanity
    if !r2.edges.is_empty() {
        for e in &r2.edges {
            assert!(e.weight >= 0.0 && e.weight <= 1.0, "edge weight in 0..=1");
        }
    }

    // processing_time_ms is a non-negative type; no need to assert tautology
}
