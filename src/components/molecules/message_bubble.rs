// Message bubble molecule component combining card and button atoms
use crate::components::atoms::{Button, ButtonSize, ButtonVariant, Card, CardVariant};
use leptos::prelude::*;

/// Message bubble for chat conversations
#[component]
pub fn MessageBubble(
    #[prop(default = "user".to_string())] sender: String,
    content: String,
    #[prop(optional)] timestamp: Option<String>,
    #[prop(default = false)] is_user: bool,
    #[prop(default = false)] is_loading: bool,
    #[prop(optional)] on_copy: Option<Callback<String>>,
    #[prop(optional)] on_regenerate: Option<Callback<()>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let handle_copy = {
        let content = content.clone();
        move || {
            if let Some(handler) = on_copy {
                handler.run(content.clone());
            }
        }
    };

    let bubble_class = if is_user {
        "ml-auto bg-primary text-primary-content"
    } else {
        "mr-auto bg-base-200"
    };

    view! {
        <div class={format!("flex flex-col gap-2 max-w-[80%] {}", class.unwrap_or_default())}>
            <div class={format!("flex items-start gap-2 {}", if is_user { "flex-row-reverse" } else { "" })}>
                // Avatar
                <div class="avatar placeholder">
                    <div class="bg-neutral text-neutral-content rounded-full w-8 h-8">
                        <span class="text-xs">
                            {if is_user { "U" } else { "A" }}
                        </span>
                    </div>
                </div>

                // Message content
                <div class={format!("flex flex-col gap-1 {}", if is_user { "items-end" } else { "items-start" })}>
                    // Sender name and timestamp
                    <div class="flex items-center gap-2 text-xs text-base-content/60">
                        <span>{sender}</span>
                        {timestamp.map(|ts| view! {
                            <span>{"•"}</span>
                            <span>{ts}</span>
                        })}
                    </div>

                    // Message bubble
                    <Card
                        variant={CardVariant::Default}
                        class={format!("p-3 rounded-2xl {}", bubble_class)}
                    >
                        {if is_loading {
                            view! {
                                <div class="flex items-center gap-2">
                                    <span class="loading loading-dots loading-sm"></span>
                                    <span class="text-sm opacity-70">Thinking...</span>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="whitespace-pre-wrap text-sm leading-relaxed">
                                    {content}
                                </div>
                            }.into_any()
                        }}
                    </Card>

                    // Action buttons
                    {(!is_loading && !is_user).then(|| view! {
                        <div class="flex items-center gap-1 mt-1">
                            <Button
                                variant={ButtonVariant::Ghost}
                                size={ButtonSize::Small}
                                on_click={Callback::new(move |_| handle_copy())}
                                class={"text-xs".to_string()}
                            >
                                <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path>
                                </svg>
                                "Copy"
                            </Button>

                            {on_regenerate.map(|handler| view! {
                                <Button
                                    variant={ButtonVariant::Ghost}
                                    size={ButtonSize::Small}
                                    on_click={Callback::new(move |_| handler.run(()))}
                                    class={"text-xs".to_string()}
                                >
                                    <svg class="w-3 h-3 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
                                    </svg>
                                    "Regenerate"
                                </Button>
                            })}
                        </div>
                    })}
                </div>
            </div>
        </div>
    }
}

/// System message bubble for notifications and status updates
#[component]
pub fn SystemMessage(
    content: String,
    #[prop(default = "info".to_string())] message_type: String, // info, warning, error, success
    #[prop(optional)] timestamp: Option<String>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let (icon, color_class) = match message_type.as_str() {
        "warning" => ("⚠️", "alert-warning"),
        "error" => ("❌", "alert-error"),
        "success" => ("✅", "alert-success"),
        _ => ("ℹ️", "alert-info"),
    };

    view! {
        <div class={format!("flex justify-center my-4 {}", class.unwrap_or_default())}>
            <div class={format!("alert {} max-w-md", color_class)}>
                <span class="text-lg">{icon}</span>
                <div class="flex flex-col">
                    <span class="text-sm">{content}</span>
                    {timestamp.map(|ts| view! {
                        <span class="text-xs opacity-70">{ts}</span>
                    })}
                </div>
            </div>
        </div>
    }
}

/// Typing indicator for showing when someone is typing
#[component]
pub fn TypingIndicator(
    #[prop(default = "Assistant".to_string())] sender: String,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    view! {
        <div class={format!("flex items-start gap-2 max-w-[80%] {}", class.unwrap_or_default())}>
            // Avatar
            <div class="avatar placeholder">
                <div class="bg-neutral text-neutral-content rounded-full w-8 h-8">
                    <span class="text-xs">A</span>
                </div>
            </div>

            // Typing bubble
            <div class="flex flex-col gap-1">
                <div class="text-xs text-base-content/60">{sender}</div>
                <Card
                    variant={CardVariant::Default}
                    class={"p-3 rounded-2xl bg-base-200".to_string()}
                >
                    <div class="flex items-center gap-1">
                        <div class="flex gap-1">
                            <div class="w-2 h-2 bg-current rounded-full animate-bounce" style="animation-delay: 0ms"></div>
                            <div class="w-2 h-2 bg-current rounded-full animate-bounce" style="animation-delay: 150ms"></div>
                            <div class="w-2 h-2 bg-current rounded-full animate-bounce" style="animation-delay: 300ms"></div>
                        </div>
                        <span class="text-xs opacity-70 ml-2">typing...</span>
                    </div>
                </Card>
            </div>
        </div>
    }
}
