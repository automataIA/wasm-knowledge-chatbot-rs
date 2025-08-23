use leptos::prelude::*;
use leptos_meta::*;

// Modules
pub mod components;
pub mod models;
pub mod storage;
pub mod utils;
pub mod state;
pub mod ui;
pub mod webllm_binding;
pub mod advanced_graphrag;
pub mod pagerank_reranking;
pub mod error_handling;
pub mod graphrag_config;
pub mod features;

// Components
use crate::components::main_interface::MainInterface;

/// Main WebLLM Knowledge Graph chat application
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="business" />
        <Title text="WebLLM Knowledge Graph Chat" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <MainInterface />
    }
}
