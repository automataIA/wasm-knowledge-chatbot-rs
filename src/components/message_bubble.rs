use crate::models::{Message, MessageRole};
use leptos::prelude::*;

#[component]
pub fn MessageBubble(message: Message) -> impl IntoView {
    let is_user = matches!(message.role, MessageRole::User);

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
        </div>
    }
}

fn format_timestamp(timestamp: f64) -> String {
    let date = js_sys::Date::new(&timestamp.into());
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    format!("{:02}:{:02}", hours, minutes)
}
