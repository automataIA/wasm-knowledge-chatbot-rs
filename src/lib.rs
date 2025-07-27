use leptos::prelude::*;
use leptos_meta::*;

// Modules
mod components;
mod models;
mod pages;
mod storage;
mod utils;
mod webllm_binding;

// Components
use crate::components::main_interface::MainInterface;
use crate::components::theme_toggle::ThemeToggle;

/// Main WebLLM Knowledge Graph chat application
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="black" />
        <Title text="WebLLM Knowledge Graph Chat" />
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <ThemeToggle />
        <MainInterface />
    }
}
