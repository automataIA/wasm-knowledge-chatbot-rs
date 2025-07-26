use leptos::prelude::*;
use leptos::web_sys::window;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = signal("black");

    // Apply theme on startup and changes
    Effect::new(move |_| {
        if let Some(document) = window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("data-theme", theme.get());
            }
        }
    });

    let toggle_theme = move |_| {
        set_theme.update(|current| {
            *current = if *current == "black" {
                "light"
            } else {
                "black"
            };

            // Apply theme to document
            if let Some(document) = window().and_then(|w| w.document()) {
                if let Some(html) = document.document_element() {
                    let _ = html.set_attribute("data-theme", current);
                }
            }
        });
    };

    view! {
        <div class="fixed top-4 right-4 z-50">
            <button
                on:click=toggle_theme
                class="btn btn-circle btn-ghost btn-sm"
                title="Toggle theme"
            >
                <i
                    data-lucide=move || if theme.get() == "black" { "sun" } else { "moon" }
                    class="h-4 w-4"
                ></i>
            </button>
        </div>
    }
}
