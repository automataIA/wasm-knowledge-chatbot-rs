use crate::components::ui_primitives::Button;
use crate::models::Conversation;
use leptos::prelude::*;

#[component]
pub fn ConversationHistory() -> impl IntoView {
    // Mock conversations for demo
    let conversations = vec![
        Conversation::new("Rust Discussion".to_string()),
        Conversation::new("Leptos UI Implementation".to_string()),
        Conversation::new("Tailwind Configuration".to_string()),
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
                                <Button
                                    label=Signal::derive(move || conv.title.clone())
                                    variant=Signal::derive(|| "btn-ghost w-full justify-start text-left p-2 h-auto min-h-0 hover:bg-base-300 transition-colors duration-200 animate-slide-in".to_string())
                                />
                            }
                        }
                    />
                </div>
            </div>
        </div>
    }
}
