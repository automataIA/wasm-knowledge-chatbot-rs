use leptos::prelude::*;

fn classes_of(parts: &[&str]) -> String {
    parts
        .iter()
        .filter(|c| !c.is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join(" ")
}

#[component]
pub fn Button(
    #[prop(into)] label: Signal<String>,
    #[prop(optional, default = Box::new(|| {}))] on_click: Box<dyn Fn()>,
    #[prop(optional, into)] variant: Option<Signal<String>>,
    #[prop(optional, into)] size: Option<Signal<String>>,
    #[prop(optional, into)] disabled: Option<Signal<bool>>,
    #[prop(optional, into)] icon: Option<Signal<String>>,
    #[prop(optional, into)] icon_position: Option<Signal<String>>, // "left" or "right"
) -> impl IntoView {
    let class = move || {
        let v = variant.as_ref().map(|s| s.get()).unwrap_or_default();
        let z = size.as_ref().map(|s| s.get()).unwrap_or_default();
        classes_of(&["btn", &v, &z])
    };

    let is_disabled = move || disabled.as_ref().map(|d| d.get()).unwrap_or(false);
    let has_icon = move || icon.as_ref().map(|i| !i.get().is_empty()).unwrap_or(false);
    let has_label = move || !label.get().is_empty();
    let icon_pos = move || icon_position.as_ref().map(|p| p.get()).unwrap_or("left".to_string());
    
    // Determine button content layout
    let content_class = move || {
        if has_icon() && has_label() {
            "flex items-center gap-2".to_string()
        } else {
            "flex items-center justify-center".to_string()
        }
    };

    view! {
        <button
            class=move || format!("{} {}", class(), content_class())
            disabled=is_disabled
            on:click=move |_| {
                on_click();
            }
        >
            <Show when=move || has_icon() && icon_pos() == "left">
                <i data-lucide=move || icon.as_ref().map(|i| i.get()).unwrap_or_default() class="flex-shrink-0"></i>
            </Show>
            
            <Show when=move || has_label()>
                <span class="">
                    {move || label.get()}
                </span>
            </Show>
            
            <Show when=move || has_icon() && icon_pos() == "right">
                <i data-lucide=move || icon.as_ref().map(|i| i.get()).unwrap_or_default() class="flex-shrink-0"></i>
            </Show>
        </button>
    }
}

#[component]
pub fn Toggle(
    checked: ReadSignal<bool>,
    set_checked: WriteSignal<bool>,
    #[prop(optional, into)] label: Option<Signal<String>>,
    #[prop(optional, into)] disabled: Option<Signal<bool>>,
    #[prop(optional, into)] extra_class: Option<Signal<String>>,
) -> impl IntoView {
    let is_disabled = move || disabled.as_ref().map(|d| d.get()).unwrap_or(false);

    // Fixed: Remove .clone() calls - Option<Signal<String>> implements Copy
    let label_for_presence = label;
    let has_label = Signal::derive(move || label_for_presence.is_some());

    let label_for_text = label;
    let label_text =
        Signal::derive(move || label_for_text.as_ref().map(|s| s.get()).unwrap_or_default());

    let input_class = move || {
        let extra = extra_class
            .as_ref()
            .map(|s| s.get())
            .unwrap_or_default();
        if extra.is_empty() {
            "toggle rounded-full".to_string()
        } else {
            format!("toggle rounded-full {}", extra)
        }
    };

    view! {
        <div class="form-control">
            <label class="cursor-pointer label items-center gap-3">
                <input
                    type="checkbox"
                    class=input_class
                    checked=checked
                    disabled=is_disabled
                    on:input=move |ev| {
                        let input = event_target::<web_sys::HtmlInputElement>(&ev);
                        set_checked.set(input.checked());
                    }
                />
                <Show when=has_label>
                    <span class="label-text ml-3">{move || label_text.get()}</span>
                </Show>
            </label>
        </div>
    }
}

#[component]
pub fn Input(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional, into)] placeholder: Option<Signal<String>>,
    #[prop(optional, into)] disabled: Option<Signal<bool>>,
    #[prop(optional, into)] size: Option<Signal<String>>,
    #[prop(optional)] on_keypress: Option<Box<dyn Fn(leptos::ev::KeyboardEvent)>>, // Fixed: Made optional
) -> impl IntoView {
    let is_disabled = move || disabled.as_ref().map(|d| d.get()).unwrap_or(false);
    let class = move || {
        let z = size.as_ref().map(|s| s.get()).unwrap_or_default();
        classes_of(&["input", "input-bordered", "w-full", &z])
    };

    view! {
        <input
            class=class
            type="text"
            prop:value=move || value.get()
            placeholder=move || placeholder.as_ref().map(|ph| ph.get()).unwrap_or_default()
            disabled=is_disabled
            on:input=move |ev| set_value.set(event_target_value(&ev))
            on:keypress=move |ev| {
                if let Some(callback) = on_keypress.as_ref() {
                    callback(ev);
                }
            }
        />
    }
}

#[component]
pub fn Textarea(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    #[prop(optional, into)] placeholder: Option<Signal<String>>,
    #[prop(optional, into)] disabled: Option<Signal<bool>>,
    #[prop(optional, into)] rows: Option<Signal<u32>>,
) -> impl IntoView {
    let is_disabled = move || disabled.as_ref().map(|d| d.get()).unwrap_or(false);
    let placeholder_text = move || placeholder.as_ref().map(|p| p.get()).unwrap_or_default();
    let row_count = move || rows.as_ref().map(|r| r.get()).unwrap_or(3);

    view! {
        <textarea
            class="textarea textarea-bordered w-full resize-none"
            prop:value=move || value.get()
            on:input=move |ev| {
                let input = event_target::<web_sys::HtmlTextAreaElement>(&ev);
                set_value.set(input.value());
            }
            prop:placeholder=placeholder_text
            prop:disabled=is_disabled
            rows=row_count
        />
    }
}

#[component]
pub fn ProgressBar(
    value: Signal<u32>,
    #[prop(into)] max: u32,
    #[prop(optional, into)] color: Option<Signal<String>>,
) -> impl IntoView {
    let class = move || {
        let c = color.as_ref().map(|s| s.get()).unwrap_or_default();
        classes_of(&["progress", &c])
    };

    view! { <progress class=class value=move || value.get() max=max></progress> }
}

// Card component removed - unused in codebase
// If needed in the future, it can be re-added when actually used
