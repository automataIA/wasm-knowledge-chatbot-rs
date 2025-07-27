use leptos::prelude::*;
use crate::models::Conversation;

#[component]
pub fn ConversationHistory() -> impl IntoView {
    // Mock conversations for demo
    let conversations = vec![
        Conversation {
            id: "1".to_string(),
            title: "Rust Discussion".to_string(),
            messages: vec![],
            last_message: js_sys::Date::now() - 3600000.0, // 1 hour ago
        },
        Conversation {
            id: "2".to_string(),
            title: "Leptos UI Implementation".to_string(),
            messages: vec![],
            last_message: js_sys::Date::now() - 7200000.0, // 2 hours ago
        },
        Conversation {
            id: "3".to_string(),
            title: "Tailwind Configuration".to_string(),
            messages: vec![],
            last_message: js_sys::Date::now() - 86400000.0, // 1 day ago
        },
    ];

    view! {
        <div class="flex-1 overflow-y-auto custom-scrollbar">
            <div class="p-4">
                <h3 class="text-sm font-medium text-base-content/70 mb-3">"Recent Conversations"</h3>
                <div class="space-y-1">
                    <For
                        each=move || conversations.clone()
                        key=|conv| conv.id.clone()
                        children=move |conv| {
                            view! {
                                <button class="btn btn-ghost w-full justify-start text-left p-2 h-auto min-h-0 hover:bg-base-300 transition-colors duration-200 animate-slide-in">
                                    <div class="flex flex-col items-start w-full">
                                        <span class="text-sm font-medium truncate w-full">{conv.title}</span>
                                        <span class="text-xs opacity-60">"2 hours ago"</span>
                                    </div>
                                </button>
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}