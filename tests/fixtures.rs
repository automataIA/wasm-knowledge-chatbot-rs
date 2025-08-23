//! Shared test fixtures

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

pub fn sample_markdown_doc() -> &'static str {
    "# GraphRAG\n\nThis is a sample document used for tests.\n\n- item 1\n- item 2\n"
}

pub fn sample_text_doc() -> &'static str {
    "Simple plain text document for testing storage and search."
}

#[wasm_bindgen_test]
fn fixtures_smoke() {
    assert!(sample_markdown_doc().contains("GraphRAG"));
    assert!(sample_text_doc().contains("text"));
}
