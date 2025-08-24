use leptos::prelude::*;
use leptos_meta::*;

// Modules
pub mod advanced_graphrag;
pub mod components;
pub mod error_handling;
pub mod features;
pub mod graphrag_config;
pub mod models;
pub mod pagerank_reranking;
pub mod state;
pub mod storage;
pub mod ui;
pub mod utils;
pub mod webllm_binding;

// Components
use crate::components::main_interface::MainInterface;

/// Main Wasm Knowledge Chatbot application
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="business" />
        <Title text="Wasm Knowledge Chatbot" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <MainInterface />
    }
}
