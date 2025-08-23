use leptos::mount::mount_to_body;
use leptos::prelude::*;
use wasm_bindgen_test::*;
use web_sys::{window, Document};

use wasm_knowledge_chatbot_rs::features::graphrag::ui::GraphRAGPanel;
use wasm_knowledge_chatbot_rs::models::app::AppError;
use wasm_knowledge_chatbot_rs::state::{GraphRAGStateContext, GraphRAGStateProvider};

wasm_bindgen_test_configure!(run_in_browser);

fn doc() -> Document {
    window().unwrap().document().unwrap()
}

#[component]
fn ErrorSetter() -> impl IntoView {
    // Grab context and set an error after mount
    let ctx = expect_context::<GraphRAGStateContext>();
    Effect::new(move |_| {
        ctx.set_error(Some(AppError::GraphRAGError(
            "Forced error for test".into(),
        )));
    });
    view! { <div></div> }
}

#[wasm_bindgen_test]
fn graph_rag_panel_renders_error_message() {
    mount_to_body(
        || view! { <GraphRAGStateProvider><ErrorSetter/><GraphRAGPanel/></GraphRAGStateProvider> },
    );

    let d = doc();
    let body_text = d.body().unwrap().text_content().unwrap_or_default();

    // Should render user-friendly error section (warning alert)
    assert!(
        body_text.contains("GraphRAG"),
        "should include GraphRAG text from user-friendly error message"
    );
}
