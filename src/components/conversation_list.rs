#![allow(dead_code)] // Leptos component parameters trigger false positives

use leptos::prelude::*;

use crate::models::{Message, MessageRole};
use crate::storage::{ConversationInfo, ConversationStorage};
use log::info;

#[component]
pub fn ConversationList<F>(
    storage: ReadSignal<Option<ConversationStorage>>,
    on_conversation_select: F,
    refresh_signal: ReadSignal<u32>,
    current_conversation_id: ReadSignal<Option<String>>,
) -> impl IntoView
where
    F: Fn(String) + Clone + Send + 'static,
{
    let (conversations, set_conversations) = signal::<Vec<ConversationInfo>>(vec![]);
    let (loading, set_loading) = signal(false);

    // Helper function to check if conversation has user messages
    let has_user_messages = move |messages: &Vec<Message>| -> bool {
        messages
            .iter()
            .any(|msg| matches!(msg.role, MessageRole::User))
    };

    // Function to load conversations with user messages
    let load_conversations = move || {
        if let Some(ref storage) = storage.get() {
            set_loading.set(true);
            info!("🔍 Loading conversations...");

            match storage.list_conversations() {
                Ok(all_conversations) => {
                    info!("📂 Found {} total conversations", all_conversations.len());
                    let current_conv_id = current_conversation_id.get();
                    let mut valid_conversations = Vec::new();
                    let mut to_delete = Vec::new();

                    // First pass: identify conversations to keep and delete
                    for conv_info in &all_conversations {
                        match storage.load_conversation(&conv_info.id) {
                            Ok(Some(messages)) => {
                                if has_user_messages(&messages) {
                                    valid_conversations.push(conv_info.clone());
                                } else if Some(conv_info.id.clone()) != current_conv_id {
                                    to_delete.push(conv_info.id.clone());
                                }
                            }
                            Ok(None) => {
                                if Some(conv_info.id.clone()) != current_conv_id {
                                    to_delete.push(conv_info.id.clone());
                                }
                            }
                            Err(e) => {
                                log::error!(
                                    "Failed to load conversation {}: {:?}",
                                    conv_info.id,
                                    e
                                );
                            }
                        }
                    }

                    // Delete invalid conversations
                    for id in to_delete {
                        if let Err(e) = storage.delete_conversation(&id) {
                            log::error!("Failed to delete conversation {}: {:?}", id, e);
                        }
                    }

                    // Sort by most recently updated (newest first)
                    valid_conversations.sort_by(|a, b| {
                        b.updated_at
                            .partial_cmp(&a.updated_at)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });

                    info!(
                        "✅ Loaded {} valid conversations",
                        valid_conversations.len()
                    );
                    set_conversations.set(valid_conversations);
                }
                Err(e) => {
                    log::error!("Failed to list conversations: {:?}", e);
                }
            }

            set_loading.set(false);
        }
    };

    // Watch for refresh signal changes
    Effect::new(move |prev_refresh: Option<u32>| {
        let current_refresh = refresh_signal.get();

        // React to ANY change in refresh signal
        if prev_refresh != Some(current_refresh) {
            info!(
                "🔄 Refresh triggered: {} -> {}",
                prev_refresh.unwrap_or(0),
                current_refresh
            );

            if storage.get().is_some() {
                load_conversations();
            }
        }

        current_refresh // Return current value for next iteration
    });

    // Additional effect that watches both current conversation and refresh signal
    Effect::new(move |_| {
        let _ = current_conversation_id.get(); // Watch for changes
        let _ = refresh_signal.get(); // Also watch refresh signal

        if storage.get().is_some() {
            info!("📝 Current conversation or refresh changed, reloading list");
            load_conversations();
        }
    });

    view! {
        <div class="flex-1 overflow-y-auto custom-scrollbar">
            <div class="p-4">
                <div class="flex justify-between items-center mb-3">
                    <h3 class="text-sm font-medium text-base-content/70">"Recent Conversations"</h3>
                    <button
                        class="btn btn-xs btn-ghost"
                        on:click=move |_| {
                            info!("Manual refresh button clicked");
                            load_conversations();
                        }
                    >
                        "↻"
                    </button>
                </div>

                <Show when=move || loading.get()>
                    <div class="flex justify-center py-4">
                        <div class="loading loading-spinner loading-sm"></div>
                    </div>
                </Show>

                <div class="space-y-1">
                    <For
                        each=conversations
                        key=|conv| conv.id.clone()
                        children=move |conv| {
                            let id = conv.id.clone();
                            let title = conv.title.clone();
                            let updated_at = conv.updated_at;
                            let on_click = {
                                let id = id.clone();
                                let on_conversation_select = on_conversation_select.clone();
                                move |_| on_conversation_select(id.clone())
                            };
                            let date = js_sys::Date::new(&wasm_bindgen::JsValue::from(updated_at));
                            let day = date.get_date();
                            let month = date.get_month() + 1;
                            let year = date.get_full_year();
                            let hours = date.get_hours();
                            let minutes = date.get_minutes();
                            let formatted_date = format!(
                                "{:02}/{:02}/{}, {:02}:{:02}",
                                day,
                                month,
                                year,
                                hours,
                                minutes,
                            );

                            // Format timestamp to dd/mm/yyyy, HH:mm format

                            // getMonth() returns 0-11

                            view! {
                                <button
                                    class="btn btn-ghost w-full justify-start text-left p-2 h-auto min-h-0 hover:bg-base-300 transition-colors duration-200"
                                    on:click=on_click
                                >
                                    <div class="flex flex-col items-start w-full">
                                        <span class="text-sm font-medium truncate w-full">
                                            {title}
                                        </span>
                                        <span class="text-xs opacity-60">{formatted_date}</span>
                                    </div>
                                </button>
                            }
                        }
                    />
                </div>

                <Show when=move || conversations.get().is_empty() && !loading.get()>
                    <div class="text-center py-8 text-base-content/60">
                        <p class="text-sm">"No conversations yet"</p>
                        <p class="text-xs mt-1">"Click 'New Chat' to begin"</p>
                    </div>
                </Show>
            </div>
        </div>
    }
}
