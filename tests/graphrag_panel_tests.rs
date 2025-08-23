#![cfg(target_arch = "wasm32")]

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use wasm_bindgen_test::*;
use web_sys::{window, Document};

use wasm_knowledge_chatbot_rs::features::graphrag::ui::GraphRAGPanel;
use wasm_knowledge_chatbot_rs::state::GraphRAGStateProvider;

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> Document {
    window().unwrap().document().unwrap()
}

#[wasm_bindgen_test]
fn mount_panel_and_check_core_elements() {
    // Mount provider + panel
    mount_to_body(|| view! { <GraphRAGStateProvider><GraphRAGPanel/></GraphRAGStateProvider> });

    let d = doc();

    // Input exists
    assert!(
        d.query_selector("input.input").unwrap().is_some(),
        "query input should exist"
    );

    // Search button exists and is a button
    assert!(
        d.query_selector("button.btn.btn-primary")
            .unwrap()
            .is_some(),
        "search button present"
    );

    // Reindex button exists
    assert!(
        d.query_selector("button.btn.btn-ghost").unwrap().is_some(),
        "reindex button present"
    );

    // Reranking checkbox exists
    assert!(
        d.query_selector("input.checkbox").unwrap().is_some(),
        "reranking checkbox present"
    );

    // Strategy select exists
    assert!(
        d.query_selector("select.select").unwrap().is_some(),
        "strategy select present"
    );

    // Empty-state hint should be visible initially
    let _hint = d.query_selector("div").unwrap();
    // Fallback: check by substring search over body text
    let body_text = d.body().unwrap().text_content().unwrap_or_default();
    assert!(
        body_text.contains("Enter a query and press Search"),
        "empty state hint should be visible"
    );
}
