use wasm_bindgen_test::*;
use wasm_bindgen_test::wasm_bindgen_test as test;

use serde_json::Value;
use wasm_knowledge_chatbot_rs::models::graphrag::{DocumentIndex, ProcessingStatus};
use wasm_knowledge_chatbot_rs::features::graphrag::extraction::extract_entities_relations;

wasm_bindgen_test_configure!(run_in_browser);

fn doc(id: &str, title: &str, content: &str) -> DocumentIndex {
    let now = js_sys::Date::now();
    DocumentIndex {
        id: id.to_string(),
        title: title.to_string(),
        content: content.to_string(),
        file_type: "md".into(),
        size_bytes: content.len() as u64,
        created_at: now,
        indexed_at: now,
        node_count: 0,
        embedding_model: None,
        processing_status: ProcessingStatus::Completed,
    }
}

#[test]
fn chunking_ner_re_and_backrefs() {
    // Content with headings/newlines to exercise chunking; includes TitleCase tokens and RE patterns
    let content = r#"
# Intro
Alice is a Engineer. Alice works at OpenAI.

## Details
Bob is a Researcher. Bob works at Acme Corp.
"#;

    let d = doc("d1", "Test", content);
    let (nodes, edges) = extract_entities_relations(&[d]);

    // We expect a document node and at least entity nodes for Alice, Engineer, Bob, Researcher, OpenAI, Acme Corp
    let mut have_doc = false;
    let mut entity_ids: Vec<String> = vec![];
    for n in &nodes {
        if n.node_type == "document" { have_doc = true; }
        if n.node_type == "entity" { entity_ids.push(n.id.clone()); }
    }
    assert!(have_doc, "should create a document node");
    assert!(entity_ids.len() >= 4, "should create several entity nodes");

    // Edges should include mentions and relation edges
    assert!(edges.iter().any(|e| e.relation == "mentions"), "should include mentions edges");
    assert!(edges.iter().any(|e| e.relation == "is_a"), "should include is_a relation edges");
    assert!(edges.iter().any(|e| e.relation == "works_at"), "should include works_at relation edges");

    // Backrefs should be present on entity node metadata (best-effort check)
    let mut any_backrefs = false;
    for n in &nodes {
        if n.node_type == "entity" {
            if let Some(Value::Array(_)) = n.metadata.get("backrefs") {
                any_backrefs = true;
                break;
            }
        }
    }
    assert!(any_backrefs, "at least one entity should have backrefs array");
}
