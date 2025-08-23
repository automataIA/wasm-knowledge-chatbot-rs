use gloo_timers::future::sleep;
use std::time::Duration;
use wasm_bindgen_test::*;

use wasm_knowledge_chatbot_rs::features::graphrag::Retriever;
use wasm_knowledge_chatbot_rs::models::graphrag::{RAGQuery, SearchStrategy};
use wasm_knowledge_chatbot_rs::state::GraphRAGStateContext;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(async)]
async fn test_retrieval_rerank_flag_sets() {
    // Given a simple query
    let mut q = RAGQuery::new("test rerank".to_string());

    // When reranking is enabled
    q.config.use_reranking = true;
    let r1 = Retriever::new().search(&q, SearchStrategy::Combined).await;
    assert!(
        r1.metadata.reranked,
        "expected reranked=true when use_reranking is set"
    );

    // And when reranking is disabled
    let mut q2 = RAGQuery::new("test rerank off".to_string());
    q2.config.use_reranking = false;
    let r2 = Retriever::new().search(&q2, SearchStrategy::Combined).await;
    assert!(
        !r2.metadata.reranked,
        "expected reranked=false when use_reranking is not set"
    );
}

#[wasm_bindgen_test(async)]
async fn test_state_searching_and_result_flow() {
    let ctx = GraphRAGStateContext::new();
    assert!(!ctx.searching_now(), "initial searching should be false");

    // Kick a query; even with empty index, it should resolve and flip flags
    let q = RAGQuery::new("hello".into());
    ctx.run_query(q, SearchStrategy::Combined);

    // Wait a bit for async search to complete
    sleep(Duration::from_millis(50)).await;

    // Then searching should be false and last_result populated
    assert!(
        !ctx.searching_now(),
        "searching should be false after completion"
    );
    assert!(
        ctx.last_result_now().is_some(),
        "last_result should be set after search"
    );
}
