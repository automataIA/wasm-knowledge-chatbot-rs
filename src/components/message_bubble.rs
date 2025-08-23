use crate::models::{Message, MessageRole};
use leptos::prelude::*;

#[component]
pub fn MessageBubble(message: Message) -> impl IntoView {
    let is_user = matches!(message.role, MessageRole::User);
    // Precompute provenance to avoid moving from `message` inside closures
    let provenance_items = message
        .metadata
        .as_ref()
        .and_then(|m| m.provenance.as_ref())
        .cloned()
        .unwrap_or_default();
    let has_sources = !is_user && !provenance_items.is_empty();
    let source_count = provenance_items.len();
    let show_sources = RwSignal::new(false);
    // Also precompute a sorted list for rendering
    let mut sorted = provenance_items.clone();
    sorted.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let sorted_items = sorted;
    let sources_sig: RwSignal<Vec<_>> = RwSignal::new(sorted_items);

    view! {
        <div class=move || {
            format!("chat {} animate-fade-in", if is_user { "chat-end" } else { "chat-start" })
        }>
            <div class="chat-image avatar">
                <div class="w-10 h-10 rounded-full bg-base-300 p-2 flex items-center justify-center">
                    <i
                        data-lucide=if is_user { "user" } else { "bot" }
                        class="h-8 w-8 text-base-content/70"
                    ></i>
                </div>
            </div>
            <div class=move || {
                format!(
                    "chat-bubble {} transition-all duration-200 hover:shadow-lg",
                    if is_user { "chat-bubble-primary" } else { "chat-bubble-neutral" },
                )
            }>{message.content}</div>
            <div class="chat-footer opacity-50">
                <time class="text-xs">{format_timestamp(message.timestamp)}</time>
            </div>
            <Show when=move || has_sources>
                <div class="mt-1 text-xs text-base-content/70">
                    <div class="flex items-center gap-2">
                        <button
                            class="underline hover:text-base-content transition-colors"
                            on:click=move |_| show_sources.update(|v| *v = !*v)
                        >
                            {move || if show_sources.get() { "Hide sources" } else { "Sources" }}
                            {format!(" ({})", source_count)}
                        </button>
                        <span class="px-1.5 py-0.5 rounded bg-base-300 text-[10px] tracking-wide">RAG</span>
                    </div>
                    <Show when=move || show_sources.get()>
                        {move || {
                            let items = sources_sig.get();
                            view! {
                                <ul class="mt-1 space-y-0.5">
                                    {items.into_iter().map(|a| {
                                        let pct = (a.confidence * 100.0).round() as i32;
                                        view! {
                                            <li class="flex items-center gap-2">
                                                <i data-lucide="file-text" class="h-3.5 w-3.5 opacity-70"></i>
                                                <span class="font-medium">{a.title}</span>
                                                <span class="opacity-60">{format!("{}%", pct)}</span>
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()}
                                </ul>
                            }.into_view()
                        }}
                    </Show>
                </div>
            </Show>
        </div>
    }
}

fn format_timestamp(timestamp: f64) -> String {
    let date = js_sys::Date::new(&timestamp.into());
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    format!("{:02}:{:02}", hours, minutes)
}
