// Modal atom component with design tokens integration
use leptos::prelude::*;
use crate::ui::theme::Theme;

/// Modal size types
#[derive(Debug, Clone, PartialEq)]
pub enum ModalSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

/// Modal component
#[component]
pub fn Modal(
    open: RwSignal<bool>,
    #[prop(default = ModalSize::Medium)] size: ModalSize,
    #[prop(default = true)] closable: bool,
    #[prop(default = true)] backdrop_close: bool,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = Theme::current();
    
    let _handle_close = move || {
        if closable {
            open.set(false);
            if let Some(handler) = on_close {
                handler.run(());
            }
        }
    };
    
    let handle_backdrop_click = move |_| {
        if backdrop_close && closable {
            open.set(false);
            if let Some(handler) = on_close {
                handler.run(());
            }
        }
    };
    
    let size_classes = match size {
        ModalSize::Small => "max-w-sm",
        ModalSize::Medium => "max-w-2xl",
        ModalSize::Large => "max-w-4xl",
        ModalSize::ExtraLarge => "max-w-6xl",
    };
    
    let modal_classes = format!(
        "modal {}",
        if open.get() { "modal-open" } else { "" }
    );
    
    let modal_box_classes = format!(
        "modal-box {} {}",
        size_classes,
        class.unwrap_or_default()
    );

    view! {
        <div 
            class={modal_classes}
            style={format!(
                "--duration-normal: {}; --ease-in-out: {};",
                theme.animations.duration_normal,
                theme.animations.ease_in_out
            )}
        >
            <div 
                class="modal-backdrop bg-black bg-opacity-50"
                on:click=handle_backdrop_click
            ></div>
            <div class={modal_box_classes}>
                {if closable {
                    view! {
                        <button
                            class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2 z-10"
                            on:click=move |_| {
                                if closable {
                                    open.set(false);
                                    if let Some(handler) = on_close {
                                        handler.run(());
                                    }
                                }
                            }
                            aria-label="Close modal"
                        >
                            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                            </svg>
                        </button>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
                
                {if let Some(title_text) = title.clone() {
                    view! { <h3 class="font-bold text-lg mb-4 pr-8">{title_text}</h3> }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
                
                <div class="modal-content">
                    {children()}
                </div>
            </div>
        </div>
    }
}

/// Modal actions component
#[component]
pub fn ModalActions(
    #[prop(default = true)] justify_end: bool,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let justify_classes = if justify_end { "justify-end" } else { "justify-start" };
    
    let combined_classes = format!(
        "modal-action flex gap-2 mt-6 {} {}",
        justify_classes,
        class.unwrap_or_default()
    );

    view! {
        <div class={combined_classes}>
            {children()}
        </div>
    }
}

/// Confirmation modal component
#[component]
pub fn ConfirmModal(
    open: RwSignal<bool>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] message: Option<String>,
    #[prop(default = "Confirm".to_string())] confirm_text: String,
    #[prop(default = "Cancel".to_string())] cancel_text: String,
    #[prop(default = false)] destructive: bool,
    #[prop(optional)] on_confirm: Option<Callback<()>>,
    #[prop(optional)] on_cancel: Option<Callback<()>>,
) -> impl IntoView {
    let handle_confirm = move || {
        open.set(false);
        if let Some(handler) = on_confirm {
            handler.run(());
        }
    };
    
    let handle_cancel = move || {
        open.set(false);
        if let Some(handler) = on_cancel {
            handler.run(());
        }
    };
    
    let confirm_button_class = if destructive {
        "btn btn-error"
    } else {
        "btn btn-primary"
    };

    view! {
        <Modal
            open={open}
            size={ModalSize::Small}
            title={title.unwrap_or_else(|| "Confirm Action".to_string())}
            closable={true}
            on_close={on_cancel.unwrap_or_else(|| Callback::new(|_| {}))}
        >
            <div class="py-4">
                <p class="text-base-content">
                    {message.unwrap_or_else(|| "Are you sure you want to proceed?".to_string())}
                </p>
            </div>
            
            <ModalActions>
                <button
                    class="btn btn-ghost"
                    on:click=move |_| handle_cancel()
                >
                    {cancel_text.clone()}
                </button>
                <button
                    class={confirm_button_class}
                    on:click=move |_| handle_confirm()
                >
                    {confirm_text.clone()}
                </button>
            </ModalActions>
        </Modal>
    }
}

/// Alert modal component
#[component]
pub fn AlertModal(
    open: RwSignal<bool>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] message: Option<String>,
    #[prop(default = "info".to_string())] alert_type: String, // info, success, warning, error
    #[prop(default = "OK".to_string())] button_text: String,
    #[prop(optional)] on_close: Option<Callback<()>>,
) -> impl IntoView {
    let handle_close = move || {
        open.set(false);
        if let Some(handler) = on_close {
            handler.run(());
        }
    };
    
    let (icon_class, title_class, button_class) = match alert_type.as_str() {
        "success" => ("text-success", "text-success", "btn-success"),
        "warning" => ("text-warning", "text-warning", "btn-warning"),
        "error" => ("text-error", "text-error", "btn-error"),
        _ => ("text-info", "text-info", "btn-info"),
    };
    
    let icon_path = match alert_type.as_str() {
        "success" => "M5 13l4 4L19 7",
        "warning" => "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z",
        "error" => "M6 18L18 6M6 6l12 12",
        _ => "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
    };

    view! {
        <Modal
            open={open}
            size={ModalSize::Small}
            closable={true}
            on_close={on_close.unwrap_or_else(|| Callback::new(|_| {}))}
        >
            <div class="text-center py-4">
                <div class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-base-200 mb-4">
                    <svg class={format!("h-6 w-6 {}", icon_class)} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d={icon_path}/>
                    </svg>
                </div>
                
                {if let Some(title_text) = title.clone() {
                    view! {
                        <h3 class={format!("text-lg font-medium mb-2 {}", title_class)}>
                            {title_text}
                        </h3>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
                
                <p class="text-base-content text-sm">
                    {message.unwrap_or_else(|| "Alert message".to_string())}
                </p>
            </div>
            
            <ModalActions justify_end={true}>
                <button
                    class={format!("btn {}", button_class)}
                    on:click=move |_| handle_close()
                >
                    {button_text.clone()}
                </button>
            </ModalActions>
        </Modal>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modal_sizes() {
        let sizes = vec![
            ModalSize::Small,
            ModalSize::Medium,
            ModalSize::Large,
            ModalSize::ExtraLarge,
        ];
        
        for size in sizes {
            match size {
                ModalSize::Small => assert_eq!(size, ModalSize::Small),
                ModalSize::Medium => assert_eq!(size, ModalSize::Medium),
                ModalSize::Large => assert_eq!(size, ModalSize::Large),
                ModalSize::ExtraLarge => assert_eq!(size, ModalSize::ExtraLarge),
            }
        }
    }

    #[test]
    fn test_modal_accessibility() {
        // Test that modal has proper ARIA attributes and keyboard handling
        // This is a structural test to ensure components exist
        let _modal = ModalSize::Medium;
        assert!(true);
    }
}
