// Chat input molecule component combining input and button atoms
use crate::components::atoms::{Button, ButtonSize, ButtonVariant, Input, InputVariant};
use leptos::prelude::*;

/// Enhanced chat input with send button and keyboard shortcuts
#[component]
pub fn ChatInput(
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(default = "Type your message...".to_string())] placeholder: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(optional)] on_send: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let internal_value = RwSignal::new(String::new());
    let input_value = value.unwrap_or(internal_value);

    let handle_send = {
        let iv = input_value;
        move || {
            let message = iv.get();
            if !message.trim().is_empty() {
                if let Some(handler) = on_send {
                    handler.run(message.clone());
                }
                iv.set(String::new());
            }
        }
    };

    let is_empty = move || input_value.get().trim().is_empty();

    view! {
        <div class={format!("flex gap-2 items-end {}", class.unwrap_or_default())}>
            <div class="flex-1">
                <Input
                    variant={InputVariant::Bordered}
                    value={input_value}
                    placeholder={placeholder}
                    disabled={disabled}
                    class={"resize-none".to_string()}
                />
            </div>
            <Button
                variant={ButtonVariant::Primary}
                size={ButtonSize::Medium}
                disabled={disabled || is_empty()}
                loading={loading}
                on_click={Callback::new(move |_| handle_send())}
                class={"shrink-0".to_string()}
            >
                {if loading {
                    view! {
                        <span class="loading loading-spinner loading-sm mr-2"></span>
                        "Sending..."
                    }.into_any()
                } else {
                    view! {
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                            <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z"></path>
                        </svg>
                        <span class="sr-only">Send message</span>
                    }.into_any()
                }}
            </Button>
        </div>
        <div class="text-xs text-base-content/60 mt-1">
            "Press Ctrl+Enter to send"
        </div>
    }
}

/// Compact chat input for mobile or constrained spaces
#[component]
pub fn CompactChatInput(
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(default = "Message...".to_string())] placeholder: String,
    #[prop(default = false)] disabled: bool,
    #[prop(default = false)] loading: bool,
    #[prop(optional)] on_send: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let internal_value = RwSignal::new(String::new());
    let input_value = value.unwrap_or(internal_value);

    let handle_send = {
        let iv = input_value;
        move || {
            let message = iv.get();
            if !message.trim().is_empty() {
                if let Some(handler) = on_send {
                    handler.run(message.clone());
                }
                iv.set(String::new());
            }
        }
    };

    let is_empty = move || input_value.get().trim().is_empty();

    view! {
        <div class={format!("relative {}", class.unwrap_or_default())}>
            <Input
                variant={InputVariant::Bordered}
                value={input_value}
                placeholder={placeholder}
                disabled={disabled}
                class={"pr-12".to_string()}
            />
            <Button
                variant={ButtonVariant::Ghost}
                size={ButtonSize::Small}
                disabled={disabled || is_empty()}
                loading={loading}
                on_click={Callback::new(move |_| handle_send())}
                class={"absolute right-1 top-1/2 -translate-y-1/2".to_string()}
            >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z"></path>
                </svg>
                <span class="sr-only">Send</span>
            </Button>
        </div>
    }
}
